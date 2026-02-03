use std::{
    collections::HashMap,
    io::stdout,
    time::{Duration, Instant},
};

use anyhow::Result;
use chrono::{DateTime, Local};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::{CrosstermBackend, Constraint, Direction, Layout, Terminal},
    widgets::{block::Title, Block, BorderType, Borders, Paragraph},
};

use crate::models::{Meeting, Role};

/// Represents the state of the live meeting TUI.
pub struct LiveMeetingState {
    pub start_time: DateTime<Local>,
    pub elapsed_seconds: i64,
    pub paused: bool,
    pub pause_elapsed: i64, // Seconds elapsed when paused
    pub attendees: HashMap<Role, u32>,
    pub cost_per_minute: f64,
    pub config_rates: HashMap<Role, f64>,
}

impl LiveMeetingState {
    pub fn new(
        attendees: HashMap<Role, u32>,
        cost_per_minute: f64,
        config_rates: HashMap<Role, f64>,
    ) -> Self {
        Self {
            start_time: Local::now(),
            elapsed_seconds: 0,
            paused: false,
            pause_elapsed: 0,
            attendees,
            cost_per_minute,
            config_rates,
        }
    }
}

pub fn run_tui(mut state: LiveMeetingState) -> Result<Option<Meeting>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_secs(1);

    let mut return_value = None;

    loop {
        terminal.draw(|frame| {
            let full_area = frame.size();

            // Draw the main border block first
            let block = Block::default()
                .title(Title::from(" MEETING IN PROGRESS "))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded);
            frame.render_widget(&block, full_area);

            // Get the inner area of the block for content
            let inner_area = block.inner(full_area);

            let outer_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(7), // Elapsed Time, Current Cost, Cost/Minute
                    Constraint::Min(0),    // Attendees
                    Constraint::Length(3), // Footer
                ])
                .split(inner_area);

            let main_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(5), // Elapsed Time, Current Cost, Cost/Minute
                    Constraint::Min(0),    // Attendees
                ])
                .split(outer_layout[0]);

            let elapsed_time = format!(
                "Elapsed Time: {:02}:{:02}:{:02}",
                state.elapsed_seconds / 3600,
                (state.elapsed_seconds / 60) % 60,
                state.elapsed_seconds % 60
            );

            let current_cost = state.cost_per_minute * state.elapsed_seconds as f64 / 60.0;
            let current_cost_str = format!("Current Cost:   ${:.2}", current_cost);
            let cost_per_minute_str = format!("Cost/Minute:    ${:.2}", state.cost_per_minute);

            let time_cost_content = format!("{}\n\n{}\n{}", elapsed_time, current_cost_str, cost_per_minute_str);
            let time_cost_paragraph = Paragraph::new(time_cost_content);
            frame.render_widget(time_cost_paragraph, main_layout[0]);

            let attendees_block = Block::default()
                .title("Attendees:")
                .borders(Borders::TOP)
                .border_type(BorderType::Plain)
                .padding(ratatui::widgets::Padding::new(1, 0, 0, 0));

            let mut attendees_text = String::new();
            for (role, count) in state.attendees.iter() {
                let rate = state.config_rates.get(role).unwrap_or(&0.0);
                attendees_text.push_str(&format!("  {}x {:?}          @ ${:.2}/hr\n", count, role, rate));
            }
            let attendees_paragraph = Paragraph::new(attendees_text).block(attendees_block);
            frame.render_widget(attendees_paragraph, outer_layout[1]);

            let footer_block = Block::default()
                .borders(Borders::TOP)
                .border_type(BorderType::Plain);
            let footer_text =
                "[Space] Pause/Resume  [Q] Quit & Save  [Esc] Quit";
            let footer_paragraph = Paragraph::new(footer_text).block(footer_block);
            frame.render_widget(footer_paragraph, outer_layout[2]);
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Char('Q') => {
                            let end_time = state.start_time + chrono::Duration::seconds(state.elapsed_seconds);
                            let total_cost = state.cost_per_minute * state.elapsed_seconds as f64 / 60.0;
                            let meeting = Meeting::new(
                                state.start_time,
                                end_time,
                                state.attendees.clone(),
                                total_cost,
                            );
                            return_value = Some(meeting);
                            break;
                        }
                        KeyCode::Esc => {
                            return_value = None;
                            break;
                        }
                        KeyCode::Char(' ') => {
                            state.paused = !state.paused;
                            if state.paused {
                                state.pause_elapsed = state.elapsed_seconds;
                            } else {
                                state.start_time += chrono::Duration::seconds(state.elapsed_seconds - state.pause_elapsed);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            if !state.paused {
                state.elapsed_seconds += 1;
            }
            last_tick = Instant::now();
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(return_value)
}