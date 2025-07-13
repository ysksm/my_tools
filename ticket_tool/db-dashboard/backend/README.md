# DuckDB Dashboard Backend

Rust + Actix-web backend for the DuckDB dashboard application.

## Setup

1. Copy `.env.example` to `.env` and configure:
   ```bash
   cp .env.example .env
   ```

2. Build and run:
   ```bash
   cargo build
   cargo run
   ```

## API Endpoints

- `GET /api/health` - Health check
- `GET /api/schema` - Get database schema information
- `GET /api/data?table={table_name}` - Get table data as Parquet
- `POST /api/query` - Execute SQL query (SELECT only)

## Environment Variables

- `DB_PATH` - Path to DuckDB database file (default: `dashboard.db`)
- `PORT` - Server port (default: `8080`)
- `HOST` - Server host (default: `127.0.0.1`)
- `RUST_LOG` - Log level (default: `info`)