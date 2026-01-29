# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.5] - 2026-01-27

### CI

- Fixed an issue in the release workflow.

## [0.2.4] - 2026-01-27

### CI

- Fixed an issue in the release workflow.

## [0.2.3] - 2026-01-27

### CI

- Fixed an issue in the release workflow.

## [0.2.2] - 2026-01-27

### Documentation

- Added `GEMINI.md` to provide comprehensive project context for AI interactions.
- Added `RUST_WALKTHROUGH.md` with a detailed explanation of the project's codebase and Rust concepts.

## [0.2.1] - 2026-01-27

### CI

- Implemented a manual GitHub Actions release workflow to build and release binaries.
- Fixed the package name in the release workflow.

## [0.2.0] - 2026-01-24

### Added

- **Retroactive Meeting Logging**: Implemented the `meeting-meter log` command to allow users to record past meetings.
- **Flexible Time & Duration Parsing**: The `log` command can accept multiple time formats (e.g., "2pm", "14:30") and duration formats (e.g., "90m", "1.5h", "1h30m").
- **Cost Calculation for Logged Meetings**: Automatically calculates and displays the total cost of the logged meeting based on attendee roles and duration.
- **Persistent Storage**: Logged meetings are saved to a JSON file, building a history of past meetings.
- New `time_parser` module for handling all time-related string parsing.

### Changed

- Updated `Meeting` model to support completed meetings with fixed start and end times.
- Refactored `Calculator` to calculate cost based on start and end times instead of just a duration in minutes.
