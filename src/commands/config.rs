use crate::config::ConfigManager;
use crate::models::Role;
use anyhow::{Result, anyhow};

pub fn set_rate(role_str: &str, rate: f64) -> Result<()> {
    let role = parse_role(role_str)?;
    let config_mgr = ConfigManager::new()?;
    config_mgr.set_rate(role, rate)?;

    println!("✓ Set {} rate to ${:.2}/hr", role, rate);

    Ok(())
}

pub fn list_rates() -> Result<()> {
    let config_mgr = ConfigManager::new()?;
    let config = config_mgr.load()?;

    println!("\nConfigured Hourly Rates:");
    println!("{:-<50}", "");

    let mut roles: Vec<_> = config.rates.iter().collect();
    roles.sort_by_key(|(_, rate)| -(**rate as i64)); // Sort by rate descending

    for (role, rate) in roles {
        println!("{:<30} ${:>8.2}/hr", format!("{}", role), rate);
    }

    println!("\nConfig file: {}", config_mgr.config_path().display());

    Ok(())
}

pub fn reset() -> Result<()> {
    let config_mgr = ConfigManager::new()?;
    config_mgr.reset()?;

    println!("✓ Reset all rates to default values");

    Ok(())
}

fn parse_role(s: &str) -> Result<Role> {
    let normalized = s.to_lowercase().replace(" ", "_").replace("-", "_");

    match normalized.as_str() {
        "engineer" | "eng" => Ok(Role::Engineer),
        "senior_engineer" | "senior_eng" | "sr_engineer" => Ok(Role::SeniorEngineer),
        "staff_engineer" | "staff_eng" => Ok(Role::StaffEngineer),
        "principal_engineer" | "principal_eng" | "principal" => Ok(Role::PrincipalEngineer),
        "product_manager" | "pm" => Ok(Role::ProductManager),
        "senior_pm" | "senior_product_manager" | "sr_pm" => Ok(Role::SeniorPm),
        "director_pm" | "director_of_pm" | "pm_director" => Ok(Role::DirectorPm),
        "designer" | "design" => Ok(Role::Designer),
        "senior_designer" | "sr_designer" => Ok(Role::SeniorDesigner),
        "analyst" => Ok(Role::Analyst),
        "senior_analyst" | "sr_analyst" => Ok(Role::SeniorAnalyst),
        "director" | "dir" => Ok(Role::Director),
        "vp" | "vice_president" => Ok(Role::Vp),
        "executive" | "exec" | "c_level" => Ok(Role::Executive),
        "generic" => Ok(Role::Generic),
        _ => Err(anyhow!(
            "Unknown role: {}. Try one of: engineer, pm, designer, analyst, director, vp, executive",
            s
        )),
    }
}
