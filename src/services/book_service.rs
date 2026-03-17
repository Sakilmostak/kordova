use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    errors::{AppError, Result},
    models::{Book, BookCopy, CreateBookRequest, PaginatedResponse, PaginationInfo, SearchBooksRequest},
};

pub struct BookService {
    db: PgPool,
}

impl BookService {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    pub async fn create_book(&self, request: CreateBookRequest) -> Result<Book> {
        let book = sqlx::query_as::<_, Book>(
            r#"
            INSERT INTO books (isbn_13, isbn_10, title, subtitle, authors, publisher, publication_year, 
                              edition, language, pages, format, genre, subjects, description, cover_image_url)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
            RETURNING *
            "#
        )
        .bind(&request.isbn_13)
        .bind(&request.isbn_10)
        .bind(&request.title)
        .bind(&request.subtitle)
        .bind(&request.authors)
        .bind(&request.publisher)
        .bind(&request.publication_year)
        .bind(&request.edition)
        .bind(&request.language.unwrap_or_else(|| "en".to_string()))
        .bind(&request.pages)
        .bind(&request.format.unwrap_or_else(|| "physical".to_string()))
        .bind(&request.genre)
        .bind(&request.subjects)
        .bind(&request.description)
        .bind(&request.cover_image_url)
        .fetch_one(&self.db)
        .await?;

        Ok(book)
    }

    pub async fn get_book(&self, id: Uuid) -> Result<Book> {
        let book = sqlx::query_as::<_, Book>(
            "SELECT * FROM books WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.db)
        .await?
        .ok_or_else(|| AppError::NotFound("Book not found".to_string()))?;

        Ok(book)
    }

    pub async fn search_books(&self, request: SearchBooksRequest) -> Result<PaginatedResponse<Book>> {
        let page = request.page.unwrap_or(1);
        let limit = request.limit.unwrap_or(20).min(100); // Max 100 per page
        let offset = (page - 1) * limit;

        let mut query_builder = sqlx::QueryBuilder::new(
            "SELECT * FROM books WHERE 1=1"
        );

        // Add search filters
        if let Some(search_query) = &request.q {
            query_builder.push(" AND to_tsvector('english', title || ' ' || COALESCE(description, '')) @@ plainto_tsquery('english', ");
            query_builder.push_bind(search_query);
            query_builder.push(")");
        }

        if let Some(author) = &request.author {
            query_builder.push(" AND ");
            query_builder.push_bind(author);
            query_builder.push(" = ANY(authors)");
        }

        if let Some(genre) = &request.genre {
            query_builder.push(" AND genre = ");
            query_builder.push_bind(genre);
        }

        if let Some(year) = request.year {
            query_builder.push(" AND publication_year = ");
            query_builder.push_bind(year);
        }

        if let Some(language) = &request.language {
            query_builder.push(" AND language = ");
            query_builder.push_bind(language);
        }

        query_builder.push(" ORDER BY created_at DESC LIMIT ");
        query_builder.push_bind(limit as i64);
        query_builder.push(" OFFSET ");
        query_builder.push_bind(offset as i64);

        let books = query_builder
            .build_query_as::<Book>()
            .fetch_all(&self.db)
            .await?;

        // Get total count for pagination
        let mut count_query = sqlx::QueryBuilder::new(
            "SELECT COUNT(*) FROM books WHERE 1=1"
        );

        // Apply same filters for count
        if let Some(search_query) = &request.q {
            count_query.push(" AND to_tsvector('english', title || ' ' || COALESCE(description, '')) @@ plainto_tsquery('english', ");
            count_query.push_bind(search_query);
            count_query.push(")");
        }

        if let Some(author) = &request.author {
            count_query.push(" AND ");
            count_query.push_bind(author);
            count_query.push(" = ANY(authors)");
        }

        if let Some(genre) = &request.genre {
            count_query.push(" AND genre = ");
            count_query.push_bind(genre);
        }

        if let Some(year) = request.year {
            count_query.push(" AND publication_year = ");
            count_query.push_bind(year);
        }

        if let Some(language) = &request.language {
            count_query.push(" AND language = ");
            count_query.push_bind(language);
        }

        let total: (i64,) = count_query
            .build_query_as()
            .fetch_one(&self.db)
            .await?;

        let total_pages = ((total.0 as f64) / (limit as f64)).ceil() as u32;

        Ok(PaginatedResponse {
            data: books,
            pagination: PaginationInfo {
                page,
                limit,
                total: total.0,
                total_pages,
                has_next: page < total_pages,
                has_prev: page > 1,
            },
        })
    }

    pub async fn update_book(&self, id: Uuid, request: CreateBookRequest) -> Result<Book> {
        let book = sqlx::query_as::<_, Book>(
            r#"
            UPDATE books SET
                isbn_13 = $2, isbn_10 = $3, title = $4, subtitle = $5, authors = $6,
                publisher = $7, publication_year = $8, edition = $9, language = $10,
                pages = $11, format = $12, genre = $13, subjects = $14, description = $15,
                cover_image_url = $16, updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#
        )
        .bind(id)
        .bind(&request.isbn_13)
        .bind(&request.isbn_10)
        .bind(&request.title)
        .bind(&request.subtitle)
        .bind(&request.authors)
        .bind(&request.publisher)
        .bind(&request.publication_year)
        .bind(&request.edition)
        .bind(&request.language.unwrap_or_else(|| "en".to_string()))
        .bind(&request.pages)
        .bind(&request.format.unwrap_or_else(|| "physical".to_string()))
        .bind(&request.genre)
        .bind(&request.subjects)
        .bind(&request.description)
        .bind(&request.cover_image_url)
        .fetch_optional(&self.db)
        .await?
        .ok_or_else(|| AppError::NotFound("Book not found".to_string()))?;

        Ok(book)
    }

    pub async fn delete_book(&self, id: Uuid) -> Result<()> {
        let result = sqlx::query("DELETE FROM books WHERE id = $1")
            .bind(id)
            .execute(&self.db)
            .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Book not found".to_string()));
        }

        Ok(())
    }

    pub async fn get_book_copies(&self, book_id: Uuid) -> Result<Vec<BookCopy>> {
        let copies = sqlx::query_as::<_, BookCopy>(
            "SELECT * FROM book_copies WHERE book_id = $1 ORDER BY created_at"
        )
        .bind(book_id)
        .fetch_all(&self.db)
        .await?;

        Ok(copies)
    }
}