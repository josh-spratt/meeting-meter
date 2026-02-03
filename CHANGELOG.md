# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0] - 2025-02-02

### Changed
- **BREAKING:** Updated all default hourly rates from SF/NYC levels to Nashville, TN market averages
- Engineer: $100/hr → $43/hr (57% reduction)
- Senior Engineer: $130/hr → $66/hr (49% reduction) 
- Staff Engineer: $160/hr → $83/hr (48% reduction)
- Principal Engineer: $200/hr → $103/hr (49% reduction)
- Product Manager: $120/hr → $72/hr (40% reduction)
- Senior PM: $150/hr → $88/hr (41% reduction)
- Director PM: $200/hr → $105/hr (48% reduction)
- Designer: $110/hr → $57/hr (48% reduction)
- Senior Designer: $140/hr → $60/hr (57% reduction)
- Analyst: $90/hr → $50/hr (44% reduction)
- Senior Analyst: $115/hr → $64/hr (44% reduction)
- Director: $200/hr → $105/hr (48% reduction)
- VP: $300/hr → $106/hr (65% reduction)
- Executive: $400/hr → $139/hr (65% reduction)
- Generic: $100/hr → $43/hr (57% reduction)

### Notes
- Rates now reflect Nashville, TN tech market with 15% competitive buffer
- Average reduction: 55% across all roles
- Meeting costs will be significantly more realistic for Nashville-based teams

## [0.3.0] - 2026-01-31

### Added

- **Live Meeting Mode (TUI)**: Implemented the `meeting-meter start` command with a real-time interactive terminal user interface (TUI).
- **Real-time Cost Tracking**: The TUI displays elapsed time, current meeting cost, and cost per minute, updated every second.
- **Pause/Resume Functionality**: Users can pause and resume the live meeting tracker.
- **Save/Discard on Exit**: Meetings tracked in live mode can be saved to history or discarded upon exiting the TUI.

## [0.2.7] - 2026-01-27

### CI

- Fixed an issue in the release workflow.

## [0.2.6] - 2026-01-27

### CI

- Changed the Homebrew update step to create a pull request instead of pushing directly to main.

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
