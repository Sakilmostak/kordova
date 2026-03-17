use chrono::{NaiveDate, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    errors::{AppError, Result},
    models::{BookCopy, CheckoutRequest, CopyStatus, ReturnRequest, Transaction, TransactionType},
};

pub struct CirculationService {
    db: PgPool,
}

impl CirculationService {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    pub async fn checkout_book(&self, request: CheckoutRequest, staff_id: Uuid) -> Result<Transaction> {
        let mut tx = self.db.begin().await?;

        // Check if copy is available
        let copy = sqlx::query_as::<_, BookCopy>(
            "SELECT * FROM book_copies WHERE id = $1 AND status = 'available'"
        )
        .bind(&request.copy_id)
        .fetch_optional(&mut *tx)
        .await?
        .ok_or_else(|| AppError::BadRequest("Book copy not available".to_string()))?;

        // Check if user exists and is active
        sqlx::query("SELECT id FROM users WHERE id = $1 AND is_active = true")
            .bind(&request.user_id)
            .fetch_optional(&mut *tx)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found or inactive".to_string()))?;

        // Calculate due date (default 14 days from now)
        let checkout_date = Utc::now().date_naive();
        let due_date = request.due_date.unwrap_or_else(|| {
            checkout_date + chrono::Duration::days(14)
        });

        // Create transaction
        let transaction = sqlx::query_as::<_, Transaction>(
            r#"
            INSERT INTO transactions (copy_id, user_id, transaction_type, checkout_date, due_date, staff_id)
            VALUES ($1, $2, 'checkout', $3, $4, $5)
            RETURNING *
            "#
        )
        .bind(&request.copy_id)
        .bind(&request.user_id)
        .bind(&checkout_date)
        .bind(&due_date)
        .bind(&staff_id)
        .fetch_one(&mut *tx)
        .await?;

        // Update copy status
        sqlx::query(
            "UPDATE book_copies SET status = 'checked_out', updated_at = NOW() WHERE id = $1"
        )
        .bind(&request.copy_id)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(transaction)
    }

    pub async fn return_book(&self, request: ReturnRequest, staff_id: Uuid) -> Result<Transaction> {
        let mut tx = self.db.begin().await?;

        // Find the active checkout transaction
        let checkout_transaction = sqlx::query_as::<_, Transaction>(
            r#"
            SELECT * FROM transactions 
            WHERE copy_id = $1 AND transaction_type = 'checkout' AND return_date IS NULL
            ORDER BY created_at DESC LIMIT 1
            "#
        )
        .bind(&request.copy_id)
        .fetch_optional(&mut *tx)
        .await?
        .ok_or_else(|| AppError::BadRequest("No active checkout found for this copy".to_string()))?;

        let return_date = Utc::now().date_naive();

        // Calculate fine if overdue
        let mut fine_amount = rust_decimal::Decimal::ZERO;
        if return_date > checkout_transaction.due_date {
            let overdue_days = (return_date - checkout_transaction.due_date).num_days();
            fine_amount = rust_decimal::Decimal::new(overdue_days * 50, 2); // $0.50 per day
        }

        // Create return transaction
        let return_transaction = sqlx::query_as::<_, Transaction>(
            r#"
            INSERT INTO transactions (copy_id, user_id, transaction_type, checkout_date, due_date, return_date, fine_amount, staff_id)
            VALUES ($1, $2, 'return', $3, $4, $5, $6, $7)
            RETURNING *
            "#
        )
        .bind(&request.copy_id)
        .bind(&checkout_transaction.user_id)
        .bind(&checkout_transaction.checkout_date)
        .bind(&checkout_transaction.due_date)
        .bind(&return_date)
        .bind(&fine_amount)
        .bind(&staff_id)
        .fetch_one(&mut *tx)
        .await?;

        // Update the checkout transaction with return date
        sqlx::query(
            "UPDATE transactions SET return_date = $1, fine_amount = $2 WHERE id = $3"
        )
        .bind(&return_date)
        .bind(&fine_amount)
        .bind(&checkout_transaction.id)
        .execute(&mut *tx)
        .await?;

        // Update copy status to available
        sqlx::query(
            "UPDATE book_copies SET status = 'available', updated_at = NOW() WHERE id = $1"
        )
        .bind(&request.copy_id)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(return_transaction)
    }

    pub async fn get_user_checkouts(&self, user_id: Uuid) -> Result<Vec<Transaction>> {
        let transactions = sqlx::query_as::<_, Transaction>(
            r#"
            SELECT * FROM transactions 
            WHERE user_id = $1 AND transaction_type = 'checkout' AND return_date IS NULL
            ORDER BY due_date ASC
            "#
        )
        .bind(user_id)
        .fetch_all(&self.db)
        .await?;

        Ok(transactions)
    }

    pub async fn get_overdue_items(&self) -> Result<Vec<Transaction>> {
        let today = Utc::now().date_naive();
        
        let transactions = sqlx::query_as::<_, Transaction>(
            r#"
            SELECT * FROM transactions 
            WHERE transaction_type = 'checkout' 
            AND return_date IS NULL 
            AND due_date < $1
            ORDER BY due_date ASC
            "#
        )
        .bind(today)
        .fetch_all(&self.db)
        .await?;

        Ok(transactions)
    }
}