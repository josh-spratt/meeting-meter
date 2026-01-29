# Gemini Context: meeting-meter

This document provides context for the `meeting-meter` project, a command-line tool written in Rust.

## Project Overview

`meeting-meter` is a CLI tool designed to calculate the cost of meetings. It can log past meetings retroactively or (in a future version) track costs in real-time. The cost is calculated based on configurable hourly rates for different roles (e.g., engineer, PM).

The project is structured as a standard Rust binary application. It uses the `clap` crate for parsing command-line arguments and `serde` for handling configuration and data storage in JSON format. The `main.rs` file acts as the entry point, delegating tasks to various sub-modules located in the `src/commands/` directory.

The `README.md` indicates that some features, like the live TUI and reporting, are not yet fully implemented.

## Key Files

-   `Cargo.toml`: Defines project metadata, dependencies, and release targets.
-   `README.md`: Provides a user-facing overview, installation instructions, and command examples.
-   `src/main.rs`: The main application entry point. Parses CLI arguments and calls the appropriate command module.
-   `src/cli.rs`: Defines the entire command-line interface structure, including all subcommands and their arguments, using the `clap` crate.
-   `src/commands/`: A directory containing modules that implement the logic for each subcommand (e.g., `log.rs`, `config.rs`).
-   `src/config.rs`: Manages loading and saving configuration, such as role-based hourly rates.
-   `src/storage.rs`: Handles reading from and writing meeting history to a JSON file.
-   `.github/workflows/release.yml`: A GitHub Actions workflow for automatically building and creating releases with binaries for Linux, macOS, and Windows.

## Building and Running

### Build

```bash
# Build for development
cargo build

# Build for release
cargo build --release
```

### Run

```bash
# Run through cargo
cargo run -- <COMMAND> [ARGS]

# Example: Log a past meeting
cargo run -- log --duration "30m" --engineers 2

# Run the compiled binary directly
./target/debug/meeting-meter <COMMAND> [ARGS]
```

### Test

```bash
# Run all tests
cargo test
```

## Development Conventions

-   **CLI:** The command-line interface is defined declaratively in `src/cli.rs` using `clap`.
-   **Modularity:** Logic for each subcommand is separated into its own module within the `src/commands` directory.
-   **Configuration:** Role-based rates are stored in a `config.toml` file, managed by the `config.rs` module.
-   **Data Storage:** Meeting history is stored in a `history.json` file in a system-appropriate data directory.
-   **Releasing:** New versions are released by pushing a version tag (e.g., `vX.Y.Z`), which triggers the `release.yml` GitHub Actions workflow.
-   **Changelog:** The `CHANGELOG.md` is maintained following the "Keep a Changelog" format.
