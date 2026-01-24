use crate::models::Role;
use chrono::Duration;
use std::collections::HashMap;

pub fn parse_attendees(
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
) -> HashMap<Role, u32> {
    let mut attendees = HashMap::new();

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

    attendees
}

pub fn display_duration(duration: &Duration) -> String {
    let hours = duration.num_hours();
    let minutes = duration.num_minutes() % 60;

    if hours > 0 {
        format!("{}h {}m", hours, minutes)
    } else {
        format!("{}m", minutes)
    }
}

pub fn display_cost(amount: f64) -> String {
    format!("${:.2}", amount)
}

pub fn display_attendees(attendees: &HashMap<Role, u32>, rates: &HashMap<Role, f64>) {
    for (role, count) in attendees {
        let rate = rates.get(role).unwrap_or(&0.0);
        println!(
            "  {}x {:<18} @ ${:<6}/hr",
            count,
            role.to_string(),
            rate
        );
    }
}
