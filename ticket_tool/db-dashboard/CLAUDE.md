# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a DuckDB dashboard application with a React frontend and a planned Rust/Actix-web backend. The application allows users to create interactive dashboards from DuckDB database files.

## Architecture

- **Frontend**: React + TypeScript + Vite
- **Backend** (planned): Rust + Actix-web + DuckDB
- **Data Storage**: DuckDB files with OPFS (Origin Private File System) caching

## Development Commands

```bash
# Install dependencies
npm install

# Start development server
npm run dev

# Build for production
npm run build

# Run linting
npm run lint

# Preview production build
npm run preview

# Backend commands (from backend directory)
cargo build
cargo run
cargo test
cargo check
```

## Project Structure

- `/src/` - React components and application logic
- `/public/` - Static assets
- `task.md` - Project specifications and requirements

## Key Features (From task.md)

1. **Schema Explorer**: Display DuckDB schema information
2. **Data Viewer**: Fetch and display data in Parquet format
3. **SQL Editor**: Execute custom SQL queries
4. **OPFS Caching**: Store data locally for offline access
5. **Dashboard Builder**: Create and save custom dashboards

## Important Implementation Notes

- Data transfer between backend and frontend uses Parquet format for efficiency
- Frontend caches data in OPFS for performance and offline capability
- SQL queries are dynamically generated based on user interactions
- Dashboard configurations are persisted on the backend

## Current Status

The project is currently a fresh Vite + React + TypeScript template. The backend implementation in Rust has not been started yet.

## Development Guidelines

1. Follow existing TypeScript and React conventions
2. Use ESLint for code quality (`npm run lint`)
3. Backend API endpoints should follow RESTful conventions
4. Implement proper error handling for SQL queries and network requests
5. Consider security implications for user-submitted SQL queries