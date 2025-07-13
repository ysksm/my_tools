# DuckDB Dashboard

A web-based dashboard application for exploring and querying DuckDB databases.

## Features

- **Schema Explorer**: Browse database tables and columns
- **SQL Editor**: Write and execute SQL queries with syntax highlighting
- **Data Table**: View query results with pagination
- **Parquet Export**: Export table data in Parquet format (planned)
- **OPFS Caching**: Local data caching for offline access (planned)

## Tech Stack

- **Frontend**: React + TypeScript + Vite
- **Backend**: Rust + Actix-web + DuckDB
- **UI Components**: Monaco Editor, React Query

## Getting Started

### Prerequisites

- Node.js 18+
- Rust 1.70+
- DuckDB CLI (optional, for data import)

### Backend Setup

1. Navigate to the backend directory:
   ```bash
   cd backend
   ```

2. Copy the environment file:
   ```bash
   cp .env.example .env
   ```

3. Build and run the backend:
   ```bash
   cargo run
   ```

   The backend will start on `http://localhost:8080`

### Frontend Setup

1. Install dependencies:
   ```bash
   npm install
   ```

2. Create a `.env` file:
   ```bash
   echo "VITE_API_URL=http://localhost:8080/api" > .env
   ```

3. Start the development server:
   ```bash
   npm run dev
   ```

   The frontend will be available at `http://localhost:5173`

### Loading Test Data

1. Stop the backend server if running
2. Load the test data:
   ```bash
   duckdb backend/dashboard.db < backend/test_data.sql
   ```
3. Restart the backend server

## API Endpoints

- `GET /api/health` - Health check
- `GET /api/schema` - Get database schema
- `GET /api/data?table={name}` - Export table as Parquet
- `POST /api/query` - Execute SQL query

## Development

### Running Tests

```bash
# Backend tests
cd backend && cargo test

# Frontend tests (not yet implemented)
npm test
```

### Building for Production

```bash
# Backend
cd backend && cargo build --release

# Frontend
npm run build
```

## Roadmap

- [ ] OPFS integration for offline data caching
- [ ] Dashboard creation and saving functionality
- [ ] Dynamic SQL generation from UI interactions
- [ ] Data visualization with charts
- [ ] Export to multiple formats (CSV, JSON, etc.)

## License

MIT