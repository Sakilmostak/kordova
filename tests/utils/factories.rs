use uuid::Uuid;
use chrono::{DateTime, NaiveDate, Utc};
use library_management_system::models::*;

/// Factory for creating test users
pub struct UserFactory;

impl UserFactory {
    pub fn create_user_request(email: Option<&str>, user_type: Option<UserType>) -> CreateUserRequest {
        CreateUserRequest {
            email: email.unwrap_or("test@example.com").to_string(),
            password: "test_password".to_string(),
            first_name: "Test".to_string(),
            last_name: "User".to_string(),
            user_type: user_type.unwrap_or(UserType::Student),
            student_id: Some("STU001".to_string()),
            department: Some("Computer Science".to_string()),
            phone: Some("+1234567890".to_string()),
            address: Some("123 Test Street".to_string()),
        }
    }

    pub fn create_student() -> CreateUserRequest {
        Self::create_user_request(Some("student@example.com"), Some(UserType::Student))
    }

    pub fn create_faculty() -> CreateUserRequest {
        Self::create_user_request(Some("faculty@example.com"), Some(UserType::Faculty))
    }

    pub fn create_admin() -> CreateUserRequest {
        Self::create_user_request(Some("admin@example.com"), Some(UserType::Admin))
    }

    pub fn create_login_request(email: Option<&str>) -> LoginRequest {
        LoginRequest {
            email: email.unwrap_or("test@example.com").to_string(),
            password: "test_password".to_string(),
        }
    }
}

/// Factory for creating test books
pub struct BookFactory;

impl BookFactory {
    pub fn create_book_request(title: Option<&str>) -> CreateBookRequest {
        CreateBookRequest {
            isbn_13: Some("9781234567890".to_string()),
            isbn_10: Some("1234567890".to_string()),
            title: title.unwrap_or("Test Book").to_string(),
            subtitle: Some("A Test Book".to_string()),
            authors: vec!["Test Author".to_string()],
            publisher: Some("Test Publisher".to_string()),
            publication_year: Some(2024),
            edition: Some("1st".to_string()),
            language: Some("English".to_string()),
            pages: Some(200),
            format: Some("Hardcover".to_string()),
            genre: Some("Fiction".to_string()),
            subjects: Some(vec!["Literature".to_string(), "Modern Fiction".to_string()]),
            description: Some("A test book for testing purposes".to_string()),
            cover_image_url: Some("https://example.com/cover.jpg".to_string()),
        }
    }

    pub fn create_programming_book() -> CreateBookRequest {
        CreateBookRequest {
            isbn_13: Some("9780134685991".to_string()),
            isbn_10: Some("0134685997".to_string()),
            title: "Effective Modern C++".to_string(),
            subtitle: Some("42 Specific Ways to Improve Your Use of C++11 and C++14".to_string()),
            authors: vec!["Scott Meyers".to_string()],
            publisher: Some("O'Reilly Media".to_string()),
            publication_year: Some(2014),
            edition: Some("1st".to_string()),
            language: Some("English".to_string()),
            pages: Some(334),
            format: Some("Paperback".to_string()),
            genre: Some("Technical".to_string()),
            subjects: Some(vec!["Programming".to_string(), "C++".to_string()]),
            description: Some("Best practices for using C++11 and C++14".to_string()),
            cover_image_url: None,
        }
    }

    pub fn create_book_copy(book_id: Uuid, barcode: Option<&str>) -> (Uuid, String, String, BookCondition, CopyStatus) {
        (
            book_id,
            barcode.unwrap_or("BC001").to_string(),
            "A1-001".to_string(),
            BookCondition::Good,
            CopyStatus::Available,
        )
    }
}

/// Factory for creating test transactions
pub struct TransactionFactory;

impl TransactionFactory {
    pub fn create_checkout_request(copy_id: Uuid, user_id: Uuid) -> CheckoutRequest {
        CheckoutRequest {
            copy_id,
            user_id,
            due_date: Some(chrono::Local::now().naive_local().date() + chrono::Duration::days(14)),
        }
    }

    pub fn create_return_request(copy_id: Uuid) -> ReturnRequest {
        ReturnRequest { copy_id }
    }
}

/// Factory for creating test reservations
pub struct ReservationFactory;

impl ReservationFactory {
    pub fn create_reservation_request(book_id: Uuid) -> CreateReservationRequest {
        CreateReservationRequest { book_id }
    }
}

/// Factory for creating search requests
pub struct SearchFactory;

impl SearchFactory {
    pub fn create_search_request() -> SearchBooksRequest {
        SearchBooksRequest {
            q: None,
            author: None,
            genre: None,
            year: None,
            language: None,
            page: Some(1),
            limit: Some(10),
        }
    }

    pub fn search_by_title(title: &str) -> SearchBooksRequest {
        SearchBooksRequest {
            q: Some(title.to_string()),
            author: None,
            genre: None,
            year: None,
            language: None,
            page: Some(1),
            limit: Some(10),
        }
    }

    pub fn search_by_author(author: &str) -> SearchBooksRequest {
        SearchBooksRequest {
            q: None,
            author: Some(author.to_string()),
            genre: None,
            year: None,
            language: None,
            page: Some(1),
            limit: Some(10),
        }
    }
}