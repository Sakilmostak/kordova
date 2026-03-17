# Library Management System

A modern, high-performance Library Management System built with Rust using the Axum web framework.

## Features

- **User Management**: Registration, authentication, and role-based access control
- **Book Catalog**: Comprehensive book management with search and filtering
- **Circulation**: Checkout, return, and renewal operations
- **Reservations**: Book reservation system with queue management
- **Reports**: Analytics and reporting dashboard
- **Admin Panel**: Administrative functions and bulk operations

## Technology Stack

- **Backend**: Rust with Axum framework
- **Database**: PostgreSQL with SQLx ORM
- **Cache**: Redis for sessions and caching
- **Authentication**: JWT with Argon2id password hashing
- **API**: RESTful API design

## Quick Start

### Prerequisites

- Rust 1.70+ 
- PostgreSQL 15+
- Redis 6+

### Setup

1. Clone the repository:
```bash
git clone <repository-url>
cd library-management-system
```

2. Copy environment configuration:
```bash
cp .env.example .env
```

3. Update the `.env` file with your database and Redis URLs.

4. Install dependencies and run migrations:
```bash
cargo build
# Migrations run automatically on startup
```

5. Run the application:
```bash
cargo run
```

The server will start on `http://localhost:3000` by default.

## API Documentation

### Authentication

#### Register User
```http
POST /api/v1/auth/register
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "secure_password",
  "first_name": "John",
  "last_name": "Doe",
  "user_type": "student",
  "student_id": "STU001",
  "department": "Computer Science"
}
```

#### Login
```http
POST /api/v1/auth/login
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "secure_password"
}
```

#### Get Profile
```http
GET /api/v1/auth/profile
Authorization: Bearer <token>
```

### Books

#### Search Books
```http
GET /api/v1/books?q=rust&author=Steve%20Klabnik&page=1&limit=20
Authorization: Bearer <token>
```

#### Get Book Details
```http
GET /api/v1/books/{book_id}
Authorization: Bearer <token>
```

#### Create Book (Admin Only)
```http
POST /api/v1/books
Authorization: Bearer <token>
Content-Type: application/json

{
  "title": "The Rust Programming Language",
  "authors": ["Steve Klabnik", "Carol Nichols"],
  "isbn_13": "9781593278281",
  "publisher": "No Starch Press",
  "publication_year": 2023,
  "genre": "Programming",
  "description": "The official guide to Rust programming language."
}
```

### Circulation

#### Checkout Book (Staff/Admin)
```http
POST /api/v1/circulation/checkout
Authorization: Bearer <token>
Content-Type: application/json

{
  "copy_id": "uuid-of-book-copy",
  "user_id": "uuid-of-user",
  "due_date": "2024-04-15"
}
```

#### Return Book (Staff/Admin)
```http
POST /api/v1/circulation/return
Authorization: Bearer <token>
Content-Type: application/json

{
  "copy_id": "uuid-of-book-copy"
}
```

### Reservations

#### Create Reservation
```http
POST /api/v1/reservations/reserve
Authorization: Bearer <token>
Content-Type: application/json

{
  "book_id": "uuid-of-book"
}
```

#### Get My Reservations
```http
GET /api/v1/reservations/my-reservations
Authorization: Bearer <token>
```

## Database Schema

The system uses PostgreSQL with the following main entities:

- **users**: User accounts and profiles
- **books**: Book catalog information
- **book_copies**: Physical/digital copies of books
- **transactions**: Circulation history (checkout/return)
- **reservations**: Book reservations and queue

## User Roles

- **Student/Faculty**: Can browse books, checkout, return, and make reservations
- **Staff**: All user permissions plus manage transactions and view reports
- **Admin**: Full system access including user management and system configuration

## Development

### Running Tests
```bash
cargo test
```

### Database Migrations
Migrations are automatically applied on startup. Manual migration files are in the `migrations/` directory.

### Code Structure

```
src/
├── main.rs              # Application entry point
├── config.rs            # Configuration management
├── database.rs          # Database connection and setup
├── auth.rs              # Authentication service
├── models.rs            # Data models and DTOs
├── errors.rs            # Error handling
├── middleware.rs        # Authentication middleware
├── handlers/            # HTTP request handlers
│   ├── auth.rs
│   ├── books.rs
│   ├── circulation.rs
│   └── ...
└── services/            # Business logic services
    ├── auth_service.rs
    ├── book_service.rs
    └── ...
```

## Production Deployment

For production deployment, ensure you:

1. Use a strong JWT secret
2. Configure proper CORS settings
3. Set up HTTPS/TLS
4. Configure database connection pooling
5. Set up monitoring and logging
6. Configure backup strategies

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## License

This project is licensed under the MIT License.