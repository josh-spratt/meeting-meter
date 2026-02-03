use std::collections::HashMap;

use anyhow::Result;

use crate::{
    calculator::Calculator,
    config::ConfigManager, // Import ConfigManager
    models::Role,
    storage::Storage,
    ui::live::{run_tui, LiveMeetingState},
};

#[allow(clippy::too_many_arguments)]
pub fn run(
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
    let mut attendees: HashMap<Role, u32> = HashMap::new();

    if engineers > 0 {
        attendees.insert(Role::Engineer, engineers);
    }
    if senior_engineers > 0 {
        attendees.insert(Role::SeniorEngineer, senior_engineers);
    }
    if staff_engineers > 0 {
        attendees.insert(Role::StaffEngineer, staff_engineers);
    }
    if principal_engineers > 0 {
        attendees.insert(Role::PrincipalEngineer, principal_engineers);
    }
    if pms > 0 {
        attendees.insert(Role::ProductManager, pms);
    }
    if senior_pms > 0 {
        attendees.insert(Role::SeniorPm, senior_pms);
    }
    if director_pms > 0 {
        attendees.insert(Role::DirectorPm, director_pms);
    }
    if designers > 0 {
        attendees.insert(Role::Designer, designers);
    }
    if senior_designers > 0 {
        attendees.insert(Role::SeniorDesigner, senior_designers);
    }
    if analysts > 0 {
        attendees.insert(Role::Analyst, analysts);
    }
    if senior_analysts > 0 {
        attendees.insert(Role::SeniorAnalyst, senior_analysts);
    }
    if directors > 0 {
        attendees.insert(Role::Director, directors);
    }
    if vps > 0 {
        attendees.insert(Role::Vp, vps);
    }
    if executives > 0 {
        attendees.insert(Role::Executive, executives);
    }
    if generic > 0 {
        attendees.insert(Role::Generic, generic);
    }

    if attendees.is_empty() {
        println!("No attendees specified. Starting a meeting with no cost.");
    }

    let config_manager = ConfigManager::new()?; // Create ConfigManager instance
    let config = config_manager.load()?; // Load Config struct using ConfigManager
    let calculator = Calculator::new(&config);
    let cost_per_minute = calculator.cost_per_minute(&attendees);

    let state = LiveMeetingState::new(attendees, cost_per_minute, config.rates);

    if let Some(meeting) = run_tui(state)? {
        let storage = Storage::new()?;
        storage.save_meeting(&meeting)?;
        println!("\nMeeting saved successfully!");
    } else {
        println!("\nMeeting discarded.");
    }

    Ok(())
}
