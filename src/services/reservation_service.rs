use chrono::{Duration, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    errors::{AppError, Result},
    models::{CreateReservationRequest, Reservation, ReservationStatus},
};

pub struct ReservationService {
    db: PgPool,
}

impl ReservationService {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    pub async fn create_reservation(&self, request: CreateReservationRequest, user_id: Uuid) -> Result<Reservation> {
        let mut tx = self.db.begin().await?;

        // Check if book exists
        sqlx::query("SELECT id FROM books WHERE id = $1")
            .bind(&request.book_id)
            .fetch_optional(&mut *tx)
            .await?
            .ok_or_else(|| AppError::NotFound("Book not found".to_string()))?;

        // Check if user already has an active reservation for this book
        let existing_reservation = sqlx::query(
            "SELECT id FROM reservations WHERE book_id = $1 AND user_id = $2 AND status = 'active'"
        )
        .bind(&request.book_id)
        .bind(&user_id)
        .fetch_optional(&mut *tx)
        .await?;

        if existing_reservation.is_some() {
            return Err(AppError::Conflict("You already have a reservation for this book".to_string()));
        }

        // Get next position in queue
        let queue_position: (i64,) = sqlx::query_as(
            "SELECT COALESCE(MAX(position_in_queue), 0) + 1 FROM reservations WHERE book_id = $1 AND status = 'active'"
        )
        .bind(&request.book_id)
        .fetch_one(&mut *tx)
        .await?;

        let expiry_date = Utc::now() + Duration::days(7); // 7 days to claim

        let reservation = sqlx::query_as::<_, Reservation>(
            r#"
            INSERT INTO reservations (book_id, user_id, expiry_date, position_in_queue)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#
        )
        .bind(&request.book_id)
        .bind(&user_id)
        .bind(&expiry_date)
        .bind(queue_position.0 as i32)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(reservation)
    }

    pub async fn get_user_reservations(&self, user_id: Uuid) -> Result<Vec<Reservation>> {
        let reservations = sqlx::query_as::<_, Reservation>(
            "SELECT * FROM reservations WHERE user_id = $1 AND status = 'active' ORDER BY created_at DESC"
        )
        .bind(user_id)
        .fetch_all(&self.db)
        .await?;

        Ok(reservations)
    }

    pub async fn cancel_reservation(&self, reservation_id: Uuid, user_id: Uuid) -> Result<()> {
        let mut tx = self.db.begin().await?;

        // Get the reservation
        let reservation = sqlx::query_as::<_, Reservation>(
            "SELECT * FROM reservations WHERE id = $1 AND user_id = $2"
        )
        .bind(&reservation_id)
        .bind(&user_id)
        .fetch_optional(&mut *tx)
        .await?
        .ok_or_else(|| AppError::NotFound("Reservation not found".to_string()))?;

        // Cancel the reservation
        sqlx::query(
            "UPDATE reservations SET status = 'cancelled' WHERE id = $1"
        )
        .bind(&reservation_id)
        .execute(&mut *tx)
        .await?;

        // Update queue positions for remaining reservations
        sqlx::query(
            r#"
            UPDATE reservations 
            SET position_in_queue = position_in_queue - 1 
            WHERE book_id = $1 
            AND status = 'active' 
            AND position_in_queue > $2
            "#
        )
        .bind(&reservation.book_id)
        .bind(&reservation.position_in_queue.unwrap_or(0))
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(())
    }

    pub async fn get_reservation_queue(&self, book_id: Uuid) -> Result<Vec<Reservation>> {
        let reservations = sqlx::query_as::<_, Reservation>(
            r#"
            SELECT * FROM reservations 
            WHERE book_id = $1 AND status = 'active' 
            ORDER BY position_in_queue ASC
            "#
        )
        .bind(book_id)
        .fetch_all(&self.db)
        .await?;

        Ok(reservations)
    }

    pub async fn expire_old_reservations(&self) -> Result<u64> {
        let now = Utc::now();
        
        let result = sqlx::query(
            "UPDATE reservations SET status = 'expired' WHERE expiry_date < $1 AND status = 'active'"
        )
        .bind(now)
        .execute(&self.db)
        .await?;

        Ok(result.rows_affected())
    }
}