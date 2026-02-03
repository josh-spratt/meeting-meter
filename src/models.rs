use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Engineer,
    SeniorEngineer,
    StaffEngineer,
    PrincipalEngineer,
    ProductManager,
    SeniorPm,
    DirectorPm,
    Designer,
    SeniorDesigner,
    Analyst,
    SeniorAnalyst,
    Director,
    Vp,
    Executive,
    Generic,
}

impl Role {
    pub fn default_rate(&self) -> f64 {
        match self {
            Role::Engineer => 43.0,           // $86K/year (was $200K)
            Role::SeniorEngineer => 66.0,     // $132K/year (was $260K)
            Role::StaffEngineer => 83.0,      // $167K/year (was $320K)
            Role::PrincipalEngineer => 103.0, // $207K/year (was $400K)
            Role::ProductManager => 72.0,     // $150K/year (was $240K)
            Role::SeniorPm => 88.0,           // $184K/year (was $300K)
            Role::DirectorPm => 105.0,        // $218K/year (was $400K)
            Role::Designer => 57.0,           // $113K/year (was $220K)
            Role::SeniorDesigner => 60.0,     // $121K/year (was $280K)
            Role::Analyst => 50.0,            // $98K/year (was $180K)
            Role::SeniorAnalyst => 64.0,      // $127K/year (was $230K)
            Role::Director => 105.0,          // $218K/year (was $400K)
            Role::Vp => 106.0,                // $220K/year (was $600K)
            Role::Executive => 139.0,         // $288K/year (was $800K)
            Role::Generic => 43.0,            // $86K/year (was $200K)
        }
    }

    pub fn all_roles() -> Vec<Role> {
        vec![
            Role::Engineer,
            Role::SeniorEngineer,
            Role::StaffEngineer,
            Role::PrincipalEngineer,
            Role::ProductManager,
            Role::SeniorPm,
            Role::DirectorPm,
            Role::Designer,
            Role::SeniorDesigner,
            Role::Analyst,
            Role::SeniorAnalyst,
            Role::Director,
            Role::Vp,
            Role::Executive,
            Role::Generic,
        ]
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Role::Engineer => "Engineer",
            Role::SeniorEngineer => "Senior Engineer",
            Role::StaffEngineer => "Staff Engineer",
            Role::PrincipalEngineer => "Principal Engineer",
            Role::ProductManager => "Product Manager",
            Role::SeniorPm => "Senior PM",
            Role::DirectorPm => "Director of PM",
            Role::Designer => "Designer",
            Role::SeniorDesigner => "Senior Designer",
            Role::Analyst => "Analyst",
            Role::SeniorAnalyst => "Senior Analyst",
            Role::Director => "Director",
            Role::Vp => "VP",
            Role::Executive => "Executive",
            Role::Generic => "Generic",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meeting {
    pub id: String,
    pub start_time: DateTime<Local>,
    pub end_time: DateTime<Local>,
    pub attendees: HashMap<Role, u32>,
    pub cost: f64,
    pub notes: Option<String>,
}

impl Meeting {
    pub fn new(
        start_time: DateTime<Local>,
        end_time: DateTime<Local>,
        attendees: HashMap<Role, u32>,
        cost: f64,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            start_time,
            end_time,
            attendees,
            cost,
            notes: None,
        }
    }

    pub fn duration_minutes(&self) -> i64 {
        self.end_time
            .signed_duration_since(self.start_time)
            .num_minutes()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub rates: HashMap<Role, f64>,
    #[serde(default)]
    pub context_switch_multiplier: f64,
    #[serde(default)]
    pub include_benefits: bool,
}

impl Default for Config {
    fn default() -> Self {
        let mut rates = HashMap::new();
        for role in Role::all_roles() {
            rates.insert(role, role.default_rate());
        }

        Self {
            rates,
            context_switch_multiplier: 1.0,
            include_benefits: false,
        }
    }
}

impl Config {
    pub fn get_rate(&self, role: &Role) -> f64 {
        self.rates
            .get(role)
            .copied()
            .unwrap_or_else(|| role.default_rate())
    }
}
