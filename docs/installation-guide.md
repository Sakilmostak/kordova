# Library Management System - Installation and Deployment Guide

## Table of Contents
1. [System Requirements](#system-requirements)
2. [Development Setup](#development-setup)
3. [Production Deployment](#production-deployment)
4. [Database Setup](#database-setup)
5. [Configuration](#configuration)
6. [Security](#security)
7. [Monitoring and Maintenance](#monitoring-and-maintenance)
8. [Troubleshooting](#troubleshooting)

## System Requirements

### Minimum Requirements
- **CPU**: 2 cores, 2.4 GHz
- **RAM**: 4 GB
- **Storage**: 10 GB available space
- **Operating System**: Linux (Ubuntu 20.04+), macOS (10.15+), or Windows 10+

### Recommended Requirements
- **CPU**: 4+ cores, 3.0+ GHz  
- **RAM**: 8+ GB
- **Storage**: 50+ GB SSD
- **Operating System**: Linux (Ubuntu 22.04 LTS)

### Software Dependencies
- **Rust**: 1.70 or higher
- **PostgreSQL**: 15.0 or higher
- **Redis**: 6.0 or higher
- **Git**: For version control

### Network Requirements
- **Ports**: 3000 (app), 5432 (postgres), 6379 (redis)
- **Firewall**: Configure appropriate rules for production
- **HTTPS**: SSL/TLS certificate for production deployment

## Development Setup

### 1. Install Rust

#### Ubuntu/Debian
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustup update
```

#### macOS
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

#### Windows
1. Download and run rustup-init.exe from https://rustup.rs/
2. Follow the installation prompts
3. Restart your terminal

### 2. Install PostgreSQL

#### Ubuntu/Debian
```bash
sudo apt update
sudo apt install postgresql postgresql-contrib
sudo systemctl start postgresql
sudo systemctl enable postgresql
```

#### macOS
```bash
brew install postgresql
brew services start postgresql
```

#### Windows
1. Download PostgreSQL installer from https://www.postgresql.org/download/
2. Run the installer and follow setup wizard
3. Note down the password for the postgres user

### 3. Install Redis

#### Ubuntu/Debian
```bash
sudo apt install redis-server
sudo systemctl start redis
sudo systemctl enable redis
```

#### macOS
```bash
brew install redis
brew services start redis
```

#### Windows
1. Download Redis from https://github.com/microsoftarchive/redis/releases
2. Extract and run redis-server.exe

### 4. Clone and Setup Project

```bash
# Clone the repository
git clone <repository-url>
cd library-management-system

# Copy environment file
cp .env.example .env

# Build the project
cargo build

# Run tests to verify setup
cargo test
```

### 5. Database Configuration

```bash
# Connect to PostgreSQL
sudo -u postgres psql

# Create database and user
CREATE DATABASE library_db;
CREATE USER library_user WITH ENCRYPTED PASSWORD 'secure_password';
GRANT ALL PRIVILEGES ON DATABASE library_db TO library_user;
\q
```

### 6. Configure Environment

Edit the `.env` file:
```env
# Database
DATABASE_URL=postgresql://library_user:secure_password@localhost/library_db

# Server
PORT=3000
HOST=0.0.0.0

# Security
JWT_SECRET=your-super-secret-jwt-key-make-it-long-and-random
PASSWORD_SALT_ROUNDS=12

# Redis
REDIS_URL=redis://127.0.0.1:6379

# Email (for notifications)
SMTP_HOST=smtp.gmail.com
SMTP_PORT=587
SMTP_USERNAME=your-email@gmail.com
SMTP_PASSWORD=your-app-password

# Logging
RUST_LOG=info
```

### 7. Run the Application

```bash
# Development mode
cargo run

# Or with auto-reload
cargo install cargo-watch
cargo watch -x run
```

The application will be available at `http://localhost:3000`

## Production Deployment

### Option 1: Traditional Server Deployment

#### 1. Prepare the Server
```bash
# Update system
sudo apt update && sudo apt upgrade -y

# Install required packages
sudo apt install curl git build-essential pkg-config libssl-dev

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install PostgreSQL and Redis
sudo apt install postgresql postgresql-contrib redis-server

# Setup firewall
sudo ufw allow ssh
sudo ufw allow 80
sudo ufw allow 443
sudo ufw enable
```

#### 2. Deploy Application
```bash
# Clone repository
git clone <repository-url> /opt/library-management-system
cd /opt/library-management-system

# Build in release mode
cargo build --release

# Create production environment file
sudo cp .env.example .env
sudo nano .env  # Configure production settings
```

#### 3. Setup Systemd Service
Create `/etc/systemd/system/library-management-system.service`:
```ini
[Unit]
Description=Library Management System
After=network.target postgresql.service redis.service

[Service]
Type=simple
User=library
Group=library
WorkingDirectory=/opt/library-management-system
ExecStart=/opt/library-management-system/target/release/library-management-system
Restart=on-failure
RestartSec=10
Environment=RUST_LOG=info

[Install]
WantedBy=multi-user.target
```

```bash
# Create user
sudo useradd -r -s /bin/false library
sudo chown -R library:library /opt/library-management-system

# Start service
sudo systemctl daemon-reload
sudo systemctl enable library-management-system
sudo systemctl start library-management-system
```

#### 4. Setup Reverse Proxy (Nginx)
```bash
sudo apt install nginx

# Create Nginx config
sudo nano /etc/nginx/sites-available/library-management-system
```

```nginx
server {
    listen 80;
    server_name your-domain.com;

    location / {
        proxy_pass http://127.0.0.1:3000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }

    # Static files (if served separately)
    location /static/ {
        alias /opt/library-management-system/static/;
        expires 1y;
        add_header Cache-Control "public, immutable";
    }
}
```

```bash
# Enable site
sudo ln -s /etc/nginx/sites-available/library-management-system /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl restart nginx
```

#### 5. Setup SSL Certificate (Let's Encrypt)
```bash
sudo apt install certbot python3-certbot-nginx
sudo certbot --nginx -d your-domain.com
sudo systemctl reload nginx
```

### Option 2: Docker Deployment

#### 1. Create Dockerfile
```dockerfile
FROM rust:1.70 AS builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src/ ./src/
COPY migrations/ ./migrations/

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/library-management-system /app/
COPY migrations/ /app/migrations/

EXPOSE 3000

CMD ["./library-management-system"]
```

#### 2. Create Docker Compose
```yaml
version: '3.8'

services:
  app:
    build: .
    ports:
      - "3000:3000"
    environment:
      - DATABASE_URL=postgresql://library_user:secure_password@db:5432/library_db
      - REDIS_URL=redis://redis:6379
      - JWT_SECRET=your-super-secret-jwt-key
      - RUST_LOG=info
    depends_on:
      - db
      - redis
    restart: unless-stopped

  db:
    image: postgres:15
    environment:
      - POSTGRES_DB=library_db
      - POSTGRES_USER=library_user
      - POSTGRES_PASSWORD=secure_password
    volumes:
      - postgres_data:/var/lib/postgresql/data
    restart: unless-stopped

  redis:
    image: redis:7-alpine
    volumes:
      - redis_data:/data
    restart: unless-stopped

  nginx:
    image: nginx:alpine
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf
      - ./ssl:/etc/ssl
    depends_on:
      - app
    restart: unless-stopped

volumes:
  postgres_data:
  redis_data:
```

#### 3. Deploy with Docker
```bash
# Build and start services
docker-compose up -d

# Check status
docker-compose ps

# View logs
docker-compose logs -f app
```

## Database Setup

### Initial Setup
The application automatically runs database migrations on startup. For manual migration management:

```bash
# Install SQLx CLI
cargo install sqlx-cli --features postgres

# Run migrations manually
sqlx migrate run --database-url postgresql://library_user:secure_password@localhost/library_db

# Create new migration
sqlx migrate add add_user_preferences

# Revert last migration
sqlx migrate revert --database-url postgresql://library_user:secure_password@localhost/library_db
```

### Database Backup and Restore

#### Backup
```bash
# Full backup
pg_dump -h localhost -U library_user -d library_db > backup.sql

# Compressed backup
pg_dump -h localhost -U library_user -d library_db | gzip > backup.sql.gz

# Automated backup script
#!/bin/bash
DATE=$(date +%Y%m%d_%H%M%S)
BACKUP_DIR="/opt/backups"
DB_NAME="library_db"
DB_USER="library_user"

mkdir -p $BACKUP_DIR
pg_dump -h localhost -U $DB_USER -d $DB_NAME | gzip > $BACKUP_DIR/backup_$DATE.sql.gz

# Keep only last 30 days of backups
find $BACKUP_DIR -name "backup_*.sql.gz" -mtime +30 -delete
```

#### Restore
```bash
# From plain SQL backup
psql -h localhost -U library_user -d library_db < backup.sql

# From compressed backup
gunzip -c backup.sql.gz | psql -h localhost -U library_user -d library_db
```

### Performance Optimization
```sql
-- Create indexes for better performance
CREATE INDEX CONCURRENTLY idx_books_title ON books(title);
CREATE INDEX CONCURRENTLY idx_books_authors ON books USING GIN(authors);
CREATE INDEX CONCURRENTLY idx_users_email ON users(email);
CREATE INDEX CONCURRENTLY idx_transactions_user_id ON transactions(user_id);
CREATE INDEX CONCURRENTLY idx_transactions_due_date ON transactions(due_date);

-- Analyze tables for query optimization
ANALYZE books;
ANALYZE users;
ANALYZE transactions;
```

## Configuration

### Environment Variables

#### Required Variables
```env
DATABASE_URL=postgresql://user:password@host:5432/database
JWT_SECRET=your-super-secret-jwt-key-make-it-long-and-random
```

#### Optional Variables
```env
# Server configuration
HOST=0.0.0.0                    # Server host (default: 127.0.0.1)
PORT=3000                       # Server port (default: 3000)

# Database
DB_MAX_CONNECTIONS=10           # Connection pool size (default: 10)
DB_MIN_CONNECTIONS=1            # Minimum connections (default: 1)
DB_ACQUIRE_TIMEOUT=30           # Connection timeout seconds (default: 30)

# Redis
REDIS_URL=redis://127.0.0.1:6379
REDIS_MAX_CONNECTIONS=10

# Security
PASSWORD_SALT_ROUNDS=12         # Argon2 cost (default: 12)
JWT_EXPIRATION=3600            # JWT expiration in seconds (default: 3600)
SESSION_TIMEOUT=1800           # Session timeout in seconds (default: 1800)

# Email
SMTP_HOST=smtp.gmail.com
SMTP_PORT=587
SMTP_USERNAME=your-email@gmail.com
SMTP_PASSWORD=your-app-password
EMAIL_FROM=noreply@library.com

# Logging
RUST_LOG=info                  # Log level (error, warn, info, debug, trace)
LOG_FORMAT=json               # Log format (json or plain)

# Features
ENABLE_REGISTRATION=true       # Allow new user registration
ENABLE_RESERVATIONS=true       # Enable book reservations
ENABLE_FINES=true             # Enable late fees
MAX_CHECKOUT_ITEMS=5          # Max books per user
LOAN_PERIOD_DAYS=14           # Default loan period
MAX_RENEWALS=2                # Maximum renewals per item
```

### Application Configuration

The application can be configured through the `config.rs` module. Key configuration areas:

#### Checkout Limits
```rust
// In src/config.rs
pub struct CheckoutLimits {
    pub student: u32,        // 5 books
    pub faculty: u32,        // 10 books 
    pub staff: u32,          // 10 books
    pub admin: u32,          // No limit
}
```

#### Loan Periods
```rust
pub struct LoanPeriods {
    pub student_days: i32,   // 14 days
    pub faculty_days: i32,   // 28 days
    pub staff_days: i32,     // 28 days
}
```

## Security

### Security Checklist

#### Application Security
- [ ] Strong JWT secret key (minimum 32 characters)
- [ ] Secure password hashing (Argon2id)
- [ ] Input validation on all endpoints
- [ ] Rate limiting configured
- [ ] CORS properly configured
- [ ] SQL injection prevention (using prepared statements)

#### Database Security
- [ ] Database user has minimum required privileges
- [ ] Database not accessible from internet
- [ ] Regular security updates applied
- [ ] Backup encryption enabled
- [ ] Connection encryption (SSL/TLS)

#### Infrastructure Security
- [ ] Firewall configured (only required ports open)
- [ ] SSH key-based authentication
- [ ] Regular security updates
- [ ] Log monitoring enabled
- [ ] Intrusion detection system
- [ ] SSL/TLS certificate (Let's Encrypt or commercial)

#### Best Practices
```bash
# Secure file permissions
sudo chmod 600 /opt/library-management-system/.env
sudo chown library:library /opt/library-management-system/.env

# Secure PostgreSQL
sudo -u postgres psql
ALTER USER library_user SET search_path = library_schema, public;
REVOKE ALL ON SCHEMA public FROM PUBLIC;
GRANT USAGE ON SCHEMA public TO library_user;

# Setup fail2ban for SSH protection
sudo apt install fail2ban
sudo systemctl enable fail2ban
sudo systemctl start fail2ban
```

### Security Configuration

#### JWT Configuration
```env
# Use a strong, random secret
JWT_SECRET=$(openssl rand -base64 64)
JWT_EXPIRATION=3600  # 1 hour
```

#### CORS Configuration
```rust
// In src/main.rs
let cors = CorsLayer::new()
    .allow_origin("https://your-domain.com".parse::<HeaderValue>().unwrap())
    .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
    .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE]);
```

#### Rate Limiting
Add to your reverse proxy configuration:
```nginx
# Nginx rate limiting
limit_req_zone $binary_remote_addr zone=api:10m rate=10r/s;

server {
    location /api/ {
        limit_req zone=api burst=20 nodelay;
        proxy_pass http://127.0.0.1:3000;
    }
}
```

## Monitoring and Maintenance

### Health Checks

The application provides several health check endpoints:

```bash
# Basic health check
curl http://localhost:3000/

# Database health
curl http://localhost:3000/health/db

# Redis health  
curl http://localhost:3000/health/redis

# System metrics
curl http://localhost:3000/metrics
```

### Logging

#### Log Configuration
```env
# Development
RUST_LOG=debug

# Production
RUST_LOG=info,library_management_system=debug

# Structured logging
LOG_FORMAT=json
```

#### Log Aggregation
For production, consider using log aggregation:

```yaml
# docker-compose.yml addition
  promtail:
    image: grafana/promtail:latest
    volumes:
      - /var/log:/var/log
      - ./promtail-config.yml:/etc/promtail/config.yml
    command: -config.file=/etc/promtail/config.yml
```

### Monitoring Stack

#### Prometheus + Grafana Setup
```yaml
# docker-compose-monitoring.yml
version: '3.8'

services:
  prometheus:
    image: prom/prometheus:latest
    ports:
      - "9090:9090"
    volumes:
      - ./prometheus.yml:/etc/prometheus/prometheus.yml

  grafana:
    image: grafana/grafana:latest
    ports:
      - "3001:3000"
    environment:
      - GF_SECURITY_ADMIN_PASSWORD=admin
    volumes:
      - grafana_data:/var/lib/grafana

volumes:
  grafana_data:
```

#### Prometheus Configuration
```yaml
# prometheus.yml
global:
  scrape_interval: 15s

scrape_configs:
  - job_name: 'library-management-system'
    static_configs:
      - targets: ['app:3000']
    metrics_path: '/metrics'
```

### Backup Strategy

#### Automated Backup Script
```bash
#!/bin/bash
# /opt/scripts/backup.sh

set -e

DATE=$(date +%Y%m%d_%H%M%S)
BACKUP_DIR="/opt/backups"
RETENTION_DAYS=30

# Create backup directory
mkdir -p $BACKUP_DIR

# Database backup
pg_dump -h localhost -U library_user -d library_db | gzip > $BACKUP_DIR/db_backup_$DATE.sql.gz

# Application files backup
tar -czf $BACKUP_DIR/app_backup_$DATE.tar.gz -C /opt library-management-system

# Redis backup
redis-cli --rdb $BACKUP_DIR/redis_backup_$DATE.rdb

# Clean old backups
find $BACKUP_DIR -name "*backup_*.gz" -mtime +$RETENTION_DAYS -delete
find $BACKUP_DIR -name "*backup_*.tar.gz" -mtime +$RETENTION_DAYS -delete
find $BACKUP_DIR -name "*backup_*.rdb" -mtime +$RETENTION_DAYS -delete

echo "Backup completed: $DATE"
```

#### Setup Cron Job
```bash
# Add to crontab
crontab -e

# Daily backup at 2 AM
0 2 * * * /opt/scripts/backup.sh >> /var/log/backup.log 2>&1
```

### Performance Monitoring

#### Key Metrics to Monitor
- **Response Time**: API endpoint response times
- **Throughput**: Requests per second
- **Error Rate**: 4xx/5xx responses
- **Database**: Connection pool usage, query times
- **Memory**: Application memory usage
- **CPU**: System CPU usage
- **Disk**: Storage usage and I/O

#### Alerting Rules
```yaml
# prometheus-alerts.yml
groups:
  - name: library-management-system
    rules:
      - alert: HighResponseTime
        expr: histogram_quantile(0.95, http_request_duration_seconds_bucket) > 2
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: High response time detected

      - alert: DatabaseConnectionsHigh
        expr: database_connections_active / database_connections_max > 0.8
        for: 2m
        labels:
          severity: critical
        annotations:
          summary: Database connection pool nearly exhausted
```

## Troubleshooting

### Common Issues

#### Application Won't Start
```bash
# Check logs
sudo journalctl -u library-management-system -f

# Common fixes:
# 1. Check database connection
psql -h localhost -U library_user -d library_db

# 2. Check Redis connection
redis-cli ping

# 3. Verify environment variables
cat /opt/library-management-system/.env

# 4. Check file permissions
ls -la /opt/library-management-system/
```

#### Database Connection Issues
```bash
# Test connection
psql -h localhost -U library_user -d library_db

# Check PostgreSQL status
sudo systemctl status postgresql

# Check logs
sudo tail -f /var/log/postgresql/postgresql-15-main.log

# Common fixes:
# 1. Restart PostgreSQL
sudo systemctl restart postgresql

# 2. Check pg_hba.conf
sudo nano /etc/postgresql/15/main/pg_hba.conf

# 3. Check postgresql.conf
sudo nano /etc/postgresql/15/main/postgresql.conf
```

#### Redis Connection Issues
```bash
# Test Redis
redis-cli ping

# Check status
sudo systemctl status redis

# Check configuration
sudo nano /etc/redis/redis.conf

# Restart Redis
sudo systemctl restart redis
```

#### High Memory Usage
```bash
# Check memory usage
free -h
top -p $(pgrep library-management-system)

# Possible solutions:
# 1. Reduce database connection pool size
# 2. Optimize database queries
# 3. Add more RAM
# 4. Enable swap if needed
```

#### Slow Performance
```bash
# Check database performance
sudo -u postgres psql library_db

# Analyze slow queries
SELECT query, mean_time, calls 
FROM pg_stat_statements 
ORDER BY mean_time DESC LIMIT 10;

# Check indexes
\d+ books  # Check table indexes

# Optimize database
VACUUM ANALYZE;
REINDEX DATABASE library_db;
```

### Diagnostic Commands

```bash
# System health
systemctl status library-management-system
ps aux | grep library
netstat -tlnp | grep 3000

# Logs
journalctl -u library-management-system --since "1 hour ago"
tail -f /var/log/nginx/error.log

# Database
sudo -u postgres psql -c "SELECT * FROM pg_stat_activity;"
sudo -u postgres psql -c "SELECT * FROM pg_stat_database WHERE datname = 'library_db';"

# Network
curl -I http://localhost:3000/
telnet localhost 3000
```

### Getting Help

#### Support Channels
- **Documentation**: Check this guide and API documentation
- **Logs**: Always include relevant log excerpts
- **GitHub Issues**: Create detailed bug reports
- **Community**: Discord/Slack channels (if available)

#### What to Include in Bug Reports
1. **Environment**: OS, Rust version, database version
2. **Configuration**: Relevant environment variables (redact secrets)
3. **Logs**: Error messages and stack traces
4. **Steps**: How to reproduce the issue
5. **Expected vs Actual**: What should happen vs what happened

---

For additional support, please refer to the project repository or contact the development team.