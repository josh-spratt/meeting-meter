use anyhow::{anyhow, Result};
use chrono::{DateTime, Duration, Local};
use std::collections::HashMap;

use crate::{
    calculator::Calculator,
    config::ConfigManager,
    helpers::{self, display_attendees, display_cost, display_duration},
    models::{Meeting, Role},
    storage::Storage,
    time_parser,
};

#[allow(clippy::too_many_arguments)]
pub fn run(
    start: Option<String>,
    end: Option<String>,
    duration: Option<String>,
    engineers: u32,
    senior_engineers: u32,
    staff_engineers: u32,
    principal_engineers: u32,
    pms: u32,
    senior_pms: u32,
    director_pms: u32,
    designers: u32,
    senior_designers: u32,
    analysts: u32,
    senior_analysts: u32,
    directors: u32,
    vps: u32,
    executives: u32,
    generic: u32,
) -> Result<()> {
    let config_manager = ConfigManager::new()?;
    let config = config_manager.load()?;

    let attendees = helpers::parse_attendees(
        engineers,
        senior_engineers,
        staff_engineers,
        principal_engineers,
        pms,
        senior_pms,
        director_pms,
        designers,
        senior_designers,
        analysts,
        senior_analysts,
        directors,
        vps,
        executives,
        generic,
    );

    if attendees.is_empty() {
        return Err(anyhow!("Cannot log a meeting with no attendees. Please specify attendees using the flags."));
    }

    let (start_time, end_time) =
        calculate_start_end_times(start, end, duration)?;

    if end_time <= start_time {
        return Err(anyhow!(
            "The meeting's end time ({}) must be after its start time ({}).",
            end_time.format("%-I:%M %p"),
            start_time.format("%-I:%M %p")
        ));
    }

    let calculator = Calculator::new(&config);
    let cost = calculator.calculate_cost(start_time, end_time, &attendees);

    let meeting = Meeting::new(start_time, end_time, attendees.clone(), cost);
    let storage = Storage::new()?;
    storage.save_meeting(&meeting)?;

    print_summary(&meeting, &config.rates)?;

    Ok(())
}

fn calculate_start_end_times(
    start_str: Option<String>,
    end_str: Option<String>,
    duration_str: Option<String>,
) -> Result<(DateTime<Local>, DateTime<Local>)> {
    let now = Local::now();
    let today = now.date_naive();

    match (start_str, end_str, duration_str) {
        // Case 1: --start and --end provided
        (Some(start), Some(end), None) => {
            let start_time = time_parser::parse_time(&start)?;
            let end_time = time_parser::parse_time(&end)?;
            let start_dt = today.and_time(start_time).and_local_timezone(Local).unwrap();
            let end_dt = today.and_time(end_time).and_local_timezone(Local).unwrap();
            Ok((start_dt, end_dt))
        }
        // Case 2: --start and --duration provided
        (Some(start), None, Some(duration)) => {
            let start_time = time_parser::parse_time(&start)?;
            let duration_val = time_parser::parse_duration(&duration)?;
            let start_dt = today.and_time(start_time).and_local_timezone(Local).unwrap();
            let end_dt = start_dt + duration_val;
            Ok((start_dt, end_dt))
        }
        // Case 3: --end and --duration provided (calculate start)
        (None, Some(end), Some(duration)) => {
            let end_time = time_parser::parse_time(&end)?;
            let duration_val = time_parser::parse_duration(&duration)?;
            let end_dt = today.and_time(end_time).and_local_timezone(Local).unwrap();
            let start_dt = end_dt - duration_val;
            Ok((start_dt, end_dt))
        }
        // Case 4: Only --duration provided (assume it just ended)
        (None, None, Some(duration)) => {
            let duration_val = time_parser::parse_duration(&duration)?;
            let end_dt = now;
            let start_dt = end_dt - duration_val;
            Ok((start_dt, end_dt))
        }
        // Case 5: --start provided, but no end or duration (assume 1 hour meeting)
        (Some(start), None, None) => {
            let start_time = time_parser::parse_time(&start)?;
            let start_dt = today.and_time(start_time).and_local_timezone(Local).unwrap();
            let end_dt = start_dt + Duration::hours(1);
             println!("No end time or duration provided, assuming a 1-hour meeting.");
            Ok((start_dt, end_dt))
        }
        // Invalid combinations
        (Some(_), Some(_), Some(_)) => {
            Err(anyhow!("Invalid arguments: Cannot provide --start, --end, and --duration simultaneously. Please provide either (--start and --end) or (--start and --duration)."))
        }
        (None, Some(_), None) => {
            Err(anyhow!("Invalid arguments: Must provide --duration or --start along with --end."))
        }
        (None, None, None) => {
            Err(anyhow!("Invalid arguments: Must provide time information, e.g., --duration '60m' or --start '2pm' --end '3pm'."))
        }
    }
}


fn print_summary(meeting: &Meeting, rates: &HashMap<Role, f64>) -> Result<()> {
    let duration = meeting.end_time - meeting.start_time;

    println!("\n\u{2713} Meeting logged successfully\n");
    println!("Meeting Details:");
    println!("  Start:    {}", meeting.start_time.format("%-I:%M %p"));
    println!("  End:      {}", meeting.end_time.format("%-I:%M %p"));
    println!("  Duration: {}", display_duration(&duration));
    println!();
    println!("Attendees:");
    display_attendees(&meeting.attendees, rates);
    println!();
    println!("Total Cost: {}", display_cost(meeting.cost));

    Ok(())
}
