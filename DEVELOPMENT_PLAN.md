# meeting-meter Development Plan

This document outlines the complete development roadmap for the meeting-meter CLI tool. Each phase is designed to be independently implementable by AI agents or human developers.

## Project Overview

**Goal**: Build a Rust CLI tool that calculates the cost of meetings in real-time or retroactively, using role-based hourly rates.

**Tech Stack**:
- Rust 2021 edition
- `clap` for CLI argument parsing
- `ratatui` + `crossterm` for terminal UI
- `chrono` for time handling
- `serde` + `serde_json` for data serialization
- `directories` for cross-platform config paths

**Storage Locations**:
- Config: `~/.config/meeting-meter/config.toml`
- Data: `~/.local/share/meeting-meter/meetings.json` (or platform equivalent)

---

## Phase 1: Core Foundation ✅ COMPLETE

### Status
Completed. All foundational infrastructure is in place.

### What Was Built
- Project structure with proper module organization
- CLI command definitions using `clap` with derive macros
- Config management system with TOML persistence
- Data models: `Role`, `Meeting`, `Config`
- Storage layer for JSON-based meeting persistence
- Calculator for cost computations
- Helper functions for parsing and formatting
- Full config commands implementation

### Files Created
- `Cargo.toml` - Dependencies and project metadata
- `src/main.rs` - Entry point and command routing
- `src/cli.rs` - Command-line argument definitions
- `src/models.rs` - Core data structures
- `src/config.rs` - Config file management
- `src/storage.rs` - Meeting persistence layer
- `src/calculator.rs` - Cost calculation logic
- `src/helpers.rs` - Utility functions
- `src/commands/mod.rs` - Commands module
- `src/commands/config.rs` - Config commands (fully implemented)
- `src/commands/{start,log,history,report}.rs` - Placeholder implementations

### Testing Phase 1
```bash
cargo build
cargo run -- config list-rates
cargo run -- config set-rate engineer 150
cargo run -- config reset
```

---

## Phase 2: Retroactive Mode Implementation ✅ COMPLETE

### Status
Completed. All requirements for retroactive meeting logging are implemented.

### Goal
Implement the `meeting-meter log` command to retroactively log past meetings with flexible time input formats.

### Requirements

#### Time Parsing
Implement a robust time parser that handles multiple formats:

**Absolute Times** (for `--start` and `--end`):
- 12-hour format: "2pm", "2:30pm", "11:45am"
- 24-hour format: "14:00", "14:30", "23:45"
- Case insensitive: "2PM", "2pm", "2Pm"
- With/without colon: "2pm", "2:00pm"

**Durations** (for `--duration`):
- Minutes only: "90m", "45m"
- Hours only: "2h", "1.5h"
- Combined: "1h30m", "2h15m"
- Decimal hours: "1.5h" = 90 minutes

**Implementation Strategy**:
1. Create `src/time_parser.rs` module
2. Add functions:
   - `parse_time(s: &str) -> Result<NaiveTime>` - Parse absolute times
   - `parse_duration(s: &str) -> Result<Duration>` - Parse duration strings
   - Handle edge cases and provide clear error messages
3. Use `chrono::NaiveTime` for time-of-day parsing
4. Combine with current date to create `DateTime<Local>`

#### Meeting Creation Logic
Update `src/commands/log.rs` to:

1. **Validate Input**:
   - Require either (start + end) OR duration
   - Reject if both duration and end time are provided
   - Ensure at least one attendee is specified
   - Validate that end time is after start time

2. **Calculate Missing Values**:
   - If start + end provided: calculate duration
   - If start + duration provided: calculate end time
   - If only duration provided: use current time as end, calculate start

3. **Create Meeting**:
   - Parse attendees using `helpers::parse_attendees()`
   - Create `Meeting` struct with parsed times
   - Calculate cost using `Calculator`
   - Save to storage using `Storage::save_meeting()`

4. **Display Output**:
   - Show meeting summary with formatted times
   - Display attendee breakdown by role
   - Show calculated cost prominently
   - Include duration in human-readable format

#### Expected Output Format
```
✓ Meeting logged successfully

Meeting Details:
  Start:    2:00 PM
  End:      3:30 PM
  Duration: 1h 30m

Attendees:
  3x Engineer          @ $100/hr
  1x Product Manager   @ $120/hr

Total Cost: $510.00
```

### Files to Modify
- Create `src/time_parser.rs`
- Update `src/commands/log.rs` (replace placeholder)
- Update `src/main.rs` if needed to import time_parser

### Testing Phase 2
```bash
# Test various time formats
cargo run -- log --start "2pm" --end "3:30pm" --engineers 3 --pms 1
cargo run -- log --start "14:00" --duration "90m" --engineers 5
cargo run -- log --duration "1h30m" --directors 2 --vps 1

# Test edge cases
cargo run -- log --duration "30m" --generic 10
cargo run -- log --start "9am" --end "5pm" --engineers 2  # Long meeting

# Verify storage
cat ~/.local/share/meeting-meter/meetings.json
```

### Acceptance Criteria
- [x] All time format examples parse correctly
- [x] Meeting data persists to JSON file
- [x] Cost calculations are accurate
- [x] Error messages are clear for invalid inputs
- [x] Output formatting is clean and readable

---

## Phase 3: Live Mode with TUI

### Goal
Implement the `meeting-meter start` command with a real-time terminal UI that shows running cost.

### Requirements

#### TUI Design
Create an interactive terminal interface using `ratatui`:

**Layout**:
```
┌─────────────────────────────────────────────────────┐
│             MEETING IN PROGRESS                      │
├─────────────────────────────────────────────────────┤
│                                                       │
│  Elapsed Time:  00:23:45                             │
│                                                       │
│  Current Cost:  $237.50                              │
│                                                       │
│  Cost/Minute:   $10.00                               │
│                                                       │
├─────────────────────────────────────────────────────┤
│  Attendees:                                          │
│    3x Engineer          @ $100/hr                    │
│    1x Product Manager   @ $120/hr                    │
│                                                       │
├─────────────────────────────────────────────────────┤
│  [Space] Pause/Resume  [Q] Quit & Save  [Esc] Quit  │
└─────────────────────────────────────────────────────┘
```

**Features**:
- Real-time clock showing elapsed time (HH:MM:SS)
- Running total cost updated every second
- Cost per minute rate displayed
- List of attendees with their roles and rates
- Pause/resume functionality
- Save on quit

#### Implementation Strategy

1. **Create TUI Module** (`src/ui/live.rs`):
   - Set up `ratatui` terminal with `crossterm` backend
   - Create main render loop
   - Handle keyboard input (space, q, esc)
   - Update display every second

2. **State Management**:
   ```rust
   struct LiveMeetingState {
       start_time: DateTime<Local>,
       elapsed_seconds: i64,
       paused: bool,
       pause_elapsed: i64,  // Seconds elapsed when paused
       attendees: HashMap<Role, u32>,
       cost_per_minute: f64,
   }
   ```

3. **Update `src/commands/start.rs`**:
   - Initialize meeting state
   - Launch TUI event loop
   - Handle pause/resume logic
   - Calculate final cost on exit
   - Save to storage if user chooses "Quit & Save"

4. **Timing Logic**:
   - Use `std::time::Instant` for accurate elapsed time
   - When paused: freeze elapsed time display
   - When resumed: continue from paused time
   - Update display at 1 Hz (every second)

5. **Cost Calculation**:
   - Use `Calculator::cost_per_minute()` to get rate
   - Multiply by elapsed minutes for running total
   - Display with 2 decimal places

#### User Interactions
- **Space**: Toggle pause/resume
- **Q**: Quit and save meeting to history
- **Esc**: Quit without saving

### Files to Create/Modify
- Create `src/ui/mod.rs`
- Create `src/ui/live.rs` (main TUI implementation)
- Update `src/commands/start.rs` (replace placeholder)

### Testing Phase 3
```bash
# Start a live meeting
cargo run -- start --engineers 4 --pms 1

# Expected behavior:
# - UI appears with clock starting at 00:00:00
# - Cost increments every second
# - Space bar pauses/resumes
# - Q saves and quits
# - Esc quits without saving

# Verify saved meeting
cargo run -- history --limit 1
```

### Acceptance Criteria
- [ ] UI renders correctly with proper layout
- [ ] Clock updates every second
- [ ] Running cost calculates accurately
- [ ] Pause/resume works correctly
- [ ] Meeting saves to history on quit
- [ ] No panic on window resize
- [ ] Clean terminal restore on exit

---

## Phase 4: Reporting and History

### Goal
Implement meeting history viewing and time-based reports.

### Requirements

#### History Command
Update `src/commands/history.rs` to display past meetings:

**Output Format**:
```
Meeting History (Last 10 meetings)
═══════════════════════════════════════════════════════════════

2026-01-24  2:00 PM - 3:30 PM  (1h 30m)
  3x Engineer, 1x PM
  Cost: $510.00

2026-01-24  10:00 AM - 11:00 AM  (1h 0m)
  5x Engineer, 1x Director
  Cost: $700.00

2026-01-23  3:00 PM - 4:15 PM  (1h 15m)
  2x Senior Engineer, 1x VP
  Cost: $625.00

───────────────────────────────────────────────────────────────
Total: $1,835.00 across 3 meetings
```

**Implementation**:
1. Load meetings from storage
2. Sort by date (most recent first)
3. Display with formatted dates, times, durations
4. Show condensed attendee summary
5. Calculate and display totals

#### Report Command
Update `src/commands/report.rs` to generate time-based reports:

**Weekly Report** (`--week`):
```
Weekly Report (Jan 18 - Jan 24, 2026)
═══════════════════════════════════════════════════════════════

Total Meetings: 12
Total Time:     18h 30m
Total Cost:     $8,450.00

By Day:
  Monday    3 meetings    4h 15m    $1,950.00
  Tuesday   2 meetings    3h 0m     $1,200.00
  Wednesday 4 meetings    6h 30m    $2,800.00
  Thursday  2 meetings    3h 15m    $1,500.00
  Friday    1 meeting     1h 30m    $1,000.00

Top Roles:
  Engineer (24 attendees)           $4,800.00
  Director (8 attendees)            $2,400.00
  Product Manager (6 attendees)     $1,200.00

Average Meeting Cost: $704.17
Average Meeting Duration: 1h 32m
```

**Monthly Report** (`--month`):
Similar format but grouped by week instead of day.

**All-Time Report** (`--all`):
- Total statistics since first meeting
- Cost trends over time
- Most expensive meeting
- Longest meeting

**Implementation Strategy**:
1. Load all meetings from storage
2. Filter by date range based on flag (week/month/all)
3. Use `chrono` to determine week/month boundaries
4. Group and aggregate data
5. Calculate statistics
6. Format output with proper alignment

#### Helper Functions Needed
```rust
// In src/helpers.rs or new src/reports.rs
fn get_week_range() -> (DateTime<Local>, DateTime<Local>)
fn get_month_range() -> (DateTime<Local>, DateTime<Local>)
fn group_by_day(meetings: &[Meeting]) -> HashMap<Date, Vec<Meeting>>
fn calculate_role_totals(meetings: &[Meeting]) -> HashMap<Role, (u32, f64)>
```

### Files to Modify
- Update `src/commands/history.rs` (replace placeholder)
- Update `src/commands/report.rs` (replace placeholder)
- Optionally create `src/reports.rs` for shared report logic

### Testing Phase 4
```bash
# Add some test data first by logging meetings
cargo run -- log --duration "1h" --engineers 3 --pms 1
cargo run -- log --duration "30m" --directors 2
cargo run -- log --duration "2h" --engineers 5 --vps 1

# View history
cargo run -- history --limit 5

# Generate reports
cargo run -- report --week
cargo run -- report --month
cargo run -- report --all
```

### Acceptance Criteria
- [ ] History shows meetings in reverse chronological order
- [ ] Reports correctly filter by time period
- [ ] All cost calculations are accurate
- [ ] Formatting is clean and aligned
- [ ] Empty reports handle gracefully (no meetings found)
- [ ] Date/time formatting is consistent

---

## Phase 5: Polish and Fun Features

### Goal
Add delightful features that make the tool more engaging and useful.

### Requirements

#### 1. Cost Equivalents
Show meeting costs in terms of everyday items:

**Implementation**:
- Create `src/equivalents.rs`
- Define common items with prices:
  ```rust
  const EQUIVALENTS: &[(&str, f64)] = &[
      ("Netflix subscriptions (monthly)", 15.49),
      ("Spotify subscriptions (monthly)", 10.99),
      ("Starbucks lattes", 5.50),
      ("Chipotle burritos", 12.00),
      ("Movie tickets", 15.00),
      ("Nintendo Switch games", 60.00),
      ("Months of gym membership", 50.00),
      ("Tanks of gas", 50.00),
  ];
  ```
- Add function to calculate and format equivalents
- Display after meeting cost in log/start commands

**Output Example**:
```
Total Cost: $510.00

That's equivalent to:
  • 32 Netflix subscriptions
  • 92 Starbucks lattes
  • 42 Chipotle burritos
  • 8 Nintendo Switch games
```

#### 2. Overhead Multipliers
Add configuration options for meeting overhead:

**New Config Fields**:
```toml
[multipliers]
context_switch = 1.2  # 20% overhead for context switching
prep_time_minutes = 15  # Average prep time per meeting
```

**CLI Flags**:
```bash
# Apply context switch tax
cargo run -- log --duration "1h" --engineers 3 --context-switch 1.3

# Include prep time
cargo run -- log --duration "1h" --engineers 3 --prep-time 30m

# Include benefits/overhead
cargo run -- log --duration "1h" --engineers 3 --include-benefits
```

**Implementation**:
- Update `Config` struct in `src/models.rs`
- Update `Calculator` to apply multipliers
- Add CLI flags to log and start commands
- Show breakdown in output:
  ```
  Base Cost:              $300.00
  Prep Time (30m):        $150.00
  Context Switch (20%):   $90.00
  ─────────────────────────────────
  Total Cost:             $540.00
  ```

#### 3. Color and Formatting Improvements

**Use `colored` or `termcolor` crate** for better visuals:
- Green for success messages
- Red for costs over threshold (e.g., >$1000)
- Yellow for warnings
- Bold for important numbers
- Dim for secondary info

**Example**:
```rust
println!("{}", "✓ Meeting logged successfully".green());
println!("Total Cost: {}", format!("${:.2}", cost).red().bold());
```

#### 4. Cost Thresholds and Warnings

Alert users when meetings are expensive:
```
⚠️  WARNING: This meeting cost $2,450.00
    Consider: Could this have been an email?
    
💡 TIP: That's 40 hours of engineering time
```

**Implementation**:
- Add thresholds in config
- Check after calculating cost
- Display warnings with helpful suggestions

#### 5. Meeting Notes
Allow users to add notes to meetings:

```bash
cargo run -- log --duration "1h" --engineers 3 --notes "Sprint planning"
```

- Add `notes` field to Meeting struct (already exists!)
- Update log/start commands to accept notes
- Display in history and reports

#### 6. Export Functionality
Add CSV export for reports:

```bash
cargo run -- report --month --export report.csv
```

**CSV Format**:
```csv
Date,Start Time,End Time,Duration (min),Total Cost,Attendees
2026-01-24,14:00,15:30,90,510.00,"3x Engineer, 1x PM"
```

#### 7. Interactive Role Selection
For users who find the CLI flags tedious:

```bash
cargo run -- log --interactive
```

Prompt user for each field:
```
Start time (or press Enter for now): 2pm
Duration (or provide end time): 1h30m
How many Engineers? 3
How many Product Managers? 1
[other roles...] 
```

### Files to Create/Modify
- Create `src/equivalents.rs`
- Update `src/models.rs` for new config fields
- Update `src/calculator.rs` for multipliers
- Update `src/commands/log.rs` and `src/commands/start.rs`
- Update `src/commands/report.rs` for export
- Add `colored` or `termcolor` to `Cargo.toml`
- Optionally create `src/interactive.rs`

### Testing Phase 5
```bash
# Test equivalents
cargo run -- log --duration "1h" --engineers 10 --directors 2

# Test multipliers
cargo run -- log --duration "1h" --engineers 3 --context-switch 1.2 --prep-time 30m

# Test export
cargo run -- report --month --export january.csv
cat january.csv

# Test interactive mode
cargo run -- log --interactive
```

### Acceptance Criteria
- [ ] Equivalents display for all cost outputs
- [ ] Multipliers correctly affect calculations
- [ ] Colors improve readability
- [ ] Warnings show for expensive meetings
- [ ] CSV export works correctly
- [ ] Interactive mode is user-friendly

---

## Additional Considerations

### Error Handling
Throughout all phases:
- Use `anyhow::Result` for error propagation
- Provide clear, actionable error messages
- Never panic in user-facing code
- Validate all user inputs

### Code Quality
- Add doc comments for public functions
- Write unit tests for core logic (calculator, time parser)
- Keep functions under 50 lines when possible
- Use meaningful variable names

### Performance
- Meeting list in memory is fine (won't scale to thousands)
- Consider switching to SQLite if file size becomes an issue
- Profile TUI updates if they feel sluggish

### Platform Compatibility
- Test on Linux, macOS, and Windows
- Use `directories` crate for cross-platform paths
- Handle terminal size variations in TUI

### Future Enhancements (Not in Current Plan)
- Integration with calendar APIs (Google Calendar, Outlook)
- Team sharing/collaboration features
- Web dashboard for visualization
- Slack/Discord bot integration
- Meeting recommendations based on patterns
- Budget tracking and alerts

---

## Getting Started for AI Agents

When working on a specific phase:

1. **Read the phase requirements carefully**
2. **Check which files need to be created or modified**
3. **Review existing code in related modules**
4. **Implement following the patterns established in Phase 1**
5. **Test thoroughly using the provided test commands**
6. **Verify acceptance criteria**

Each phase is designed to be independently completable while building on previous work.
