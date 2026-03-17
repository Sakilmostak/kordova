# Library Management System - API Reference

## Base URL
```
http://localhost:3000/api/v1
```

## Authentication
All endpoints except `/auth/login` and `/auth/register` require authentication via Bearer token.

```
Authorization: Bearer <your-jwt-token>
```

## Endpoints

### Authentication (`/auth`)

#### POST `/auth/register`
Register a new user account.

**Request Body:**
```json
{
  "email": "user@example.com",
  "password": "secure_password",
  "first_name": "John",
  "last_name": "Doe",
  "user_type": "student|faculty|staff|admin",
  "student_id": "STU001",
  "department": "Computer Science",
  "phone": "+1-555-0123",
  "address": "123 Main St, City, State"
}
```

**Response:**
```json
{
  "user": {
    "id": "uuid",
    "email": "user@example.com",
    "first_name": "John",
    "last_name": "Doe",
    "user_type": "student",
    "student_id": "STU001",
    "department": "Computer Science"
  },
  "message": "User registered successfully"
}
```

#### POST `/auth/login`
Authenticate user and receive access token.

**Request Body:**
```json
{
  "email": "user@example.com",
  "password": "secure_password"
}
```

**Response:**
```json
{
  "access_token": "jwt-token-string",
  "token_type": "Bearer",
  "expires_in": 3600,
  "user": {
    "id": "uuid",
    "email": "user@example.com",
    "first_name": "John",
    "last_name": "Doe",
    "user_type": "student",
    "student_id": "STU001",
    "department": "Computer Science"
  }
}
```

#### GET `/auth/profile`
Get current user profile.

**Response:**
```json
{
  "id": "uuid",
  "email": "user@example.com",
  "first_name": "John",
  "last_name": "Doe",
  "user_type": "student",
  "student_id": "STU001",
  "department": "Computer Science"
}
```

### Books (`/books`)

#### GET `/books`
Search and list books with optional filters.

**Query Parameters:**
- `q` (string): Search query for title/description
- `author` (string): Filter by author name
- `genre` (string): Filter by genre
- `year` (integer): Filter by publication year
- `language` (string): Filter by language code
- `page` (integer): Page number (default: 1)
- `limit` (integer): Items per page (default: 20, max: 100)

**Example Request:**
```
GET /books?q=rust programming&author=Steve Klabnik&page=1&limit=10
```

**Response:**
```json
{
  "data": [
    {
      "id": "uuid",
      "isbn_13": "9781593278281",
      "title": "The Rust Programming Language",
      "authors": ["Steve Klabnik", "Carol Nichols"],
      "publisher": "No Starch Press",
      "publication_year": 2023,
      "genre": "Programming",
      "language": "en",
      "description": "The official guide...",
      "created_at": "2024-03-18T00:00:00Z"
    }
  ],
  "pagination": {
    "page": 1,
    "limit": 10,
    "total": 50,
    "total_pages": 5,
    "has_next": true,
    "has_prev": false
  }
}
```

#### GET `/books/{id}`
Get detailed information about a specific book.

**Response:**
```json
{
  "id": "uuid",
  "isbn_13": "9781593278281",
  "isbn_10": "1593278284",
  "title": "The Rust Programming Language",
  "subtitle": "Second Edition",
  "authors": ["Steve Klabnik", "Carol Nichols"],
  "publisher": "No Starch Press",
  "publication_year": 2023,
  "edition": "2nd",
  "language": "en",
  "pages": 560,
  "format": "physical",
  "genre": "Programming",
  "subjects": ["Rust", "Systems Programming"],
  "description": "The official guide to Rust programming language.",
  "cover_image_url": "https://example.com/cover.jpg",
  "created_at": "2024-03-18T00:00:00Z",
  "updated_at": "2024-03-18T00:00:00Z"
}
```

#### POST `/books` (Admin Only)
Create a new book in the catalog.

**Request Body:**
```json
{
  "title": "The Rust Programming Language",
  "authors": ["Steve Klabnik", "Carol Nichols"],
  "isbn_13": "9781593278281",
  "isbn_10": "1593278284",
  "publisher": "No Starch Press",
  "publication_year": 2023,
  "edition": "2nd",
  "language": "en",
  "pages": 560,
  "genre": "Programming",
  "subjects": ["Rust", "Systems Programming"],
  "description": "The official guide to Rust programming language.",
  "cover_image_url": "https://example.com/cover.jpg"
}
```

#### PUT `/books/{id}` (Admin Only)
Update an existing book.

#### DELETE `/books/{id}` (Admin Only)
Delete a book from the catalog.

#### GET `/books/{id}/copies`
Get all physical copies of a book.

**Response:**
```json
[
  {
    "id": "uuid",
    "book_id": "uuid",
    "barcode": "LIB001234",
    "location": "Section A, Shelf 3",
    "condition": "good",
    "acquisition_date": "2024-01-15",
    "price": 29.99,
    "status": "available",
    "notes": "First edition copy",
    "created_at": "2024-01-15T00:00:00Z"
  }
]
```

### Circulation (`/circulation`) [Coming Soon]

#### POST `/circulation/checkout`
Check out a book copy to a user.

#### POST `/circulation/return`
Return a checked-out book copy.

#### POST `/circulation/renew`
Renew a checked-out book.

#### GET `/circulation/history/{user_id}`
Get circulation history for a user.

#### GET `/circulation/overdue`
Get list of overdue items.

### Reservations (`/reservations`) [Coming Soon]

#### POST `/reservations/reserve`
Create a reservation for a book.

#### GET `/reservations/my-reservations`
Get current user's active reservations.

#### DELETE `/reservations/{id}`
Cancel a reservation.

#### GET `/reservations/queue/{book_id}`
Get reservation queue for a book.

### Reports (`/reports`) [Coming Soon]

#### GET `/reports/circulation-stats`
Get circulation statistics.

#### GET `/reports/popular-books`
Get most popular books report.

#### GET `/reports/overdue-report`
Get overdue items report.

#### GET `/reports/user-activity`
Get user activity report.

### Admin (`/admin`) [Coming Soon]

#### GET `/admin/dashboard`
Get admin dashboard data.

#### GET `/admin/system-status`
Get system health status.

#### POST `/admin/bulk-import`
Bulk import books from CSV/JSON.

## Error Responses

All error responses follow this format:

```json
{
  "error": {
    "message": "Description of the error",
    "type": "error_type_code"
  }
}
```

### HTTP Status Codes

- `200` - Success
- `201` - Created
- `400` - Bad Request (validation errors)
- `401` - Unauthorized (authentication required)
- `403` - Forbidden (insufficient permissions)
- `404` - Not Found
- `409` - Conflict (duplicate resource)
- `500` - Internal Server Error

### Error Types

- `validation_error` - Request data validation failed
- `authentication_error` - Authentication required or failed
- `authorization_error` - Insufficient permissions
- `not_found` - Requested resource not found
- `conflict` - Resource already exists
- `database_error` - Database operation failed
- `internal_error` - Internal server error