# AGENTS.md

This file contains guidelines for agentic coding agents working on the meeting-meter project.

## Project Overview

Meeting-meter is a Rust CLI tool for calculating the cost of meetings. It supports live mode, retroactive logging, role-based pricing, meeting history, and reports.

## Build and Development Commands

### Core Commands
```bash
# Build the project
cargo build

# Build optimized release version
cargo build --release

# Run the application
cargo run -- [args]

# Install locally
cargo install --path .
```

### Testing Commands
```bash
# Run all tests
cargo test

# Run a single test file/module
cargo test calculator
cargo test time_parser

# Run a specific test
cargo test test_calculate_cost
cargo test test_parse_duration_simple

# Run tests with output
cargo test -- --nocapture

# Run tests in release mode (for performance testing)
cargo test --release
```

### Linting and Formatting
```bash
# Format code (uses rustfmt with default settings)
cargo fmt

# Check code formatting without applying changes
cargo fmt -- --check

# Run Clippy lints
cargo clippy

# Run Clippy with all pedantic lints
cargo clippy -- -W clippy::pedantic

# Fix auto-fixable Clippy lints
cargo clippy --fix
```

### Other Useful Commands
```bash
# Check for compilation errors without building
cargo check

# Generate documentation
cargo doc --open

# Update dependencies
cargo update

# Clean build artifacts
cargo clean
```

## Code Style Guidelines

### Module Structure
- Use `mod` declarations at the top of `main.rs` for all modules
- Group related functionality into modules (e.g., `commands/`, `ui/`)
- Use `mod.rs` for module directories with sub-modules

### Imports
- Group imports in this order:
  1. Standard library imports (`std::*`)
  2. External crate imports
  3. Local module imports (`crate::*`)
- Use `use` statements for frequently used items
- Prefer fully qualified paths for one-off usage
- Example:
  ```rust
  use anyhow::{Context, Result};
  use chrono::{DateTime, Local};
  use std::fs;
  use crate::models::{Config, Role};
  ```

### Error Handling
- Use `anyhow::Result<T>` as the main error type
- Use `anyhow::Context` for error context: `.context("Failed to...")`
- Use `?` operator for error propagation
- Avoid `unwrap()` except in tests
- Return `Result<()>` from functions that may fail

### Naming Conventions
- Functions and variables: `snake_case`
- Types (structs, enums): `PascalCase`
- Constants: `SCREAMING_SNAKE_CASE`
- Module names: `snake_case`
- Field names in structs: `snake_case`

### Struct and Enum Patterns
- Add `#[derive(Debug, Clone)]` to most public structs
- Use `#[derive(Serialize, Deserialize)]` for data models
- Use `#[serde(rename_all = "snake_case")]` for enum serialization
- Implement `Default` for configuration structs
- Use `pub` visibility explicitly when needed

### Function Patterns
- Use `pub fn` for public functions
- Keep functions focused and small (<50 lines when possible)
- Use type annotations for function signatures
- Return `&self` or `&mut self` for methods
- Use `impl` blocks for methods on types

### Testing Patterns
- Place tests in `mod tests` blocks at the bottom of files
- Use descriptive test names with `test_` prefix
- Use `assert_eq!` and `assert!` macros
- Test both success and error cases
- Use `super::*` to import parent module items
- Example:
  ```rust
  #[cfg(test)]
  mod tests {
      use super::*;
      
      #[test]
      fn test_function_name() {
          // test code
      }
  }
  ```

### CLI and Argument Parsing
- Use `clap` with derive macros
- Use `#[command(subcommand)]` for subcommands
- Use `#[arg(long, default_value = "0")]` for CLI arguments
- Use descriptive help text in doc comments
- Handle commands in `main.rs` with match statements

### Data Storage
- Use `serde_json` for JSON serialization
- Use `toml` for configuration files
- Use `directories` crate for config directory paths
- Handle file I/O with proper error context

### Date/Time Handling
- Use `chrono` library with `DateTime<Local>`
- Use `Duration` for time differences
- Handle time parsing with custom error messages

### UI/Terminal
- Use `ratatui` for terminal UI components
- Use `crossterm` for cross-platform terminal handling
- Keep UI logic separate from business logic

### Performance Considerations
- Use `HashMap` for role-based lookups
- Prefer references over cloning when possible
- Use `&str` for string arguments when ownership isn't needed
- Consider `lazy_static` for expensive static data

### Documentation
- Add doc comments (`///`) for public items
- Use `#[command(about = "...")]` for CLI help
- Keep README updated with new features
- Use examples in doc comments when helpful

### Git and Version Control
- Use conventional commit messages (optional but helpful)
- Tag releases with `v*` format
- Keep CHANGELOG.md updated with breaking changes