# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

rata_cap is a Rust project currently in early development stage. The project structure follows standard Rust conventions using Cargo as the build system.

## Build and Development Commands

### Building the project
```bash
cargo build
cargo build --release  # For optimized release build
```

### Running the application
```bash
cargo run
```

### Testing
```bash
cargo test
cargo test -- --nocapture  # To see println! output during tests
```

### Linting and Formatting
```bash
cargo fmt       # Format code
cargo clippy    # Run linter
```

## Project Structure

This is a simple Rust binary application with:
- `src/main.rs`: Entry point containing the main function
- `Cargo.toml`: Project manifest defining dependencies and metadata
- Standard Cargo project layout with `target/` directory for build artifacts

The codebase is minimal with just a "Hello, world!" program, indicating this is either a new project or a template for future development.