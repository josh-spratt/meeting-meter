//! `time_parser.rs`
//
// This module is responsible for parsing time and duration strings
// from user input into structured data that the application can use.
// It handles various formats for absolute times (e.g., "2pm", "14:30")
// and durations (e.g., "90m", "1.5h").

use anyhow::{anyhow, Result};
use chrono::{Duration, NaiveTime};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    // Handles "1.5h", "0.5h", etc.
    static ref DECIMAL_HOURS_RE: Regex = Regex::new(r"^(?i)(\d+(?:\.\d+)?)\s*h$").unwrap();
    // Handles "1h30m", "1h", "30m"
    static ref DURATION_RE: Regex = Regex::new(r"^(?i)(?:(\d+)\s*h)?\s*(?:(\d+)\s*m)?$").unwrap();
    // Handles "2pm", "2:30pm", "14:00"
    static ref TIME_RE: Regex = Regex::new(r"^(?i)(\d{1,2})(?::(\d{2}))?\s*(am|pm)?$").unwrap();
}

/// Parses a duration string into a `chrono::Duration`.
///
/// Supports formats like:
/// - Minutes only: "90m", "45m"
/// - Hours only: "2h", "1.5h"
/// - Combined: "1h30m", "2h15m"
pub fn parse_duration(s: &str) -> Result<Duration> {
    let s = s.trim();

    // Try parsing decimal hours first, e.g., "1.5h"
    if let Some(caps) = DECIMAL_HOURS_RE.captures(s) {
        if let Some(hours_str) = caps.get(1) {
            let hours = hours_str.as_str().parse::<f64>()?;
            return Ok(Duration::minutes((hours * 60.0).round() as i64));
        }
    }

    // Fallback to humantime for "1h 30m", "90m", etc.
    match humantime::parse_duration(s) {
        Ok(std_duration) => Ok(Duration::from_std(std_duration)?),
        Err(_) => {
            // Humantime failed, try our own regex for combined h/m without spaces
            if let Some(caps) = DURATION_RE.captures(s) {
                let hours = caps.get(1).map_or(0, |m| m.as_str().parse::<i64>().unwrap_or(0));
                let minutes = caps.get(2).map_or(0, |m| m.as_str().parse::<i64>().unwrap_or(0));

                if hours > 0 || minutes > 0 {
                    return Ok(Duration::hours(hours) + Duration::minutes(minutes));
                }
            }
            Err(anyhow!("Invalid duration format: '{}'. Use formats like '1.5h', '90m', or '1h30m'.", s))
        }
    }
}


/// Parses an absolute time string into a `chrono::NaiveTime`.
///
/// Supports formats like:
/// - 12-hour format: "2pm", "2:30pm", "11:45am"
/// - 24-hour format: "14:00", "14:30"
/// - Case insensitive and flexible with colons.
pub fn parse_time(s: &str) -> Result<NaiveTime> {
    let s = s.trim();
    let caps = TIME_RE.captures(s).ok_or_else(|| anyhow!("Invalid time format: '{}'. Use formats like '2pm', '14:30', or '9:15am'.", s))?;

    let mut hour = caps.get(1).unwrap().as_str().parse::<u32>()?;
    let minute = caps.get(2).map_or(0, |m| m.as_str().parse::<u32>().unwrap_or(0));

    if minute >= 60 {
        return Err(anyhow!("Invalid minute value: {}", minute));
    }

    match caps.get(3).map(|m| m.as_str().to_lowercase()) {
        Some(ref am_pm) if am_pm == "pm" => {
            if hour < 12 {
                hour += 12;
            }
        }
        Some(ref am_pm) if am_pm == "am" => {
            if hour == 12 { // 12am is midnight
                hour = 0;
            }
        }
        // No am/pm, assume 24-hour if hour > 12, otherwise assume it's valid as is
        _ => {}
    }

    if hour >= 24 {
        return Err(anyhow!("Invalid hour value: {}", hour));
    }

    NaiveTime::from_hms_opt(hour, minute, 0).ok_or_else(|| anyhow!("Failed to construct time"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_duration_simple() {
        assert_eq!(parse_duration("30m").unwrap(), Duration::minutes(30));
        assert_eq!(parse_duration("2h").unwrap(), Duration::hours(2));
        assert_eq!(parse_duration("1h30m").unwrap(), Duration::minutes(90));
    }

    #[test]
    fn test_parse_duration_decimal() {
        assert_eq!(parse_duration("1.5h").unwrap(), Duration::minutes(90));
        assert_eq!(parse_duration("0.5h").unwrap(), Duration::minutes(30));
        assert_eq!(parse_duration("0.25h").unwrap(), Duration::minutes(15));
    }

    #[test]
    fn test_parse_duration_from_humantime() {
        assert_eq!(parse_duration("90m").unwrap(), Duration::minutes(90));
        assert_eq!(parse_duration("1m").unwrap(), Duration::minutes(1));
    }

    #[test]
    fn test_parse_time_12hr() {
        assert_eq!(parse_time("2pm").unwrap(), NaiveTime::from_hms_opt(14, 0, 0).unwrap());
        assert_eq!(parse_time("2:30pm").unwrap(), NaiveTime::from_hms_opt(14, 30, 0).unwrap());
        assert_eq!(parse_time("9:15am").unwrap(), NaiveTime::from_hms_opt(9, 15, 0).unwrap());
        assert_eq!(parse_time("12pm").unwrap(), NaiveTime::from_hms_opt(12, 0, 0).unwrap());
        assert_eq!(parse_time("12am").unwrap(), NaiveTime::from_hms_opt(0, 0, 0).unwrap());
    }

    #[test]
    fn test_parse_time_24hr() {
        assert_eq!(parse_time("14:00").unwrap(), NaiveTime::from_hms_opt(14, 0, 0).unwrap());
        assert_eq!(parse_time("23:59").unwrap(), NaiveTime::from_hms_opt(23, 59, 0).unwrap());
        assert_eq!(parse_time("00:00").unwrap(), NaiveTime::from_hms_opt(0, 0, 0).unwrap());
        assert_eq!(parse_time("9:30").unwrap(), NaiveTime::from_hms_opt(9, 30, 0).unwrap());
    }
}
