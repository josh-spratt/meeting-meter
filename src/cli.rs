use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "meeting-meter")]
#[command(about = "Calculate the cost of meetings", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Start a live meeting timer
    Start {
        #[arg(long, default_value = "0")]
        engineers: u32,
        #[arg(long, default_value = "0")]
        senior_engineers: u32,
        #[arg(long, default_value = "0")]
        staff_engineers: u32,
        #[arg(long, default_value = "0")]
        principal_engineers: u32,
        #[arg(long, default_value = "0")]
        pms: u32,
        #[arg(long, default_value = "0")]
        senior_pms: u32,
        #[arg(long, default_value = "0")]
        director_pms: u32,
        #[arg(long, default_value = "0")]
        designers: u32,
        #[arg(long, default_value = "0")]
        senior_designers: u32,
        #[arg(long, default_value = "0")]
        analysts: u32,
        #[arg(long, default_value = "0")]
        senior_analysts: u32,
        #[arg(long, default_value = "0")]
        directors: u32,
        #[arg(long, default_value = "0")]
        vps: u32,
        #[arg(long, default_value = "0")]
        executives: u32,
        #[arg(long, default_value = "0")]
        generic: u32,
    },

    /// Log a past meeting
    Log {
        /// Start time (e.g., "2pm", "14:00", "2:30pm")
        #[arg(long)]
        start: Option<String>,

        /// End time (e.g., "3pm", "15:00", "3:30pm")
        #[arg(long)]
        end: Option<String>,

        /// Duration (e.g., "90m", "1h30m", "1.5h")
        #[arg(long)]
        duration: Option<String>,

        #[arg(long, default_value = "0")]
        engineers: u32,
        #[arg(long, default_value = "0")]
        senior_engineers: u32,
        #[arg(long, default_value = "0")]
        staff_engineers: u32,
        #[arg(long, default_value = "0")]
        principal_engineers: u32,
        #[arg(long, default_value = "0")]
        pms: u32,
        #[arg(long, default_value = "0")]
        senior_pms: u32,
        #[arg(long, default_value = "0")]
        director_pms: u32,
        #[arg(long, default_value = "0")]
        designers: u32,
        #[arg(long, default_value = "0")]
        senior_designers: u32,
        #[arg(long, default_value = "0")]
        analysts: u32,
        #[arg(long, default_value = "0")]
        senior_analysts: u32,
        #[arg(long, default_value = "0")]
        directors: u32,
        #[arg(long, default_value = "0")]
        vps: u32,
        #[arg(long, default_value = "0")]
        executives: u32,
        #[arg(long, default_value = "0")]
        generic: u32,
    },

    /// Manage configuration
    Config {
        #[command(subcommand)]
        action: ConfigCommands,
    },

    /// View meeting history
    History {
        /// Number of recent meetings to show
        #[arg(short, long, default_value = "10")]
        limit: usize,
    },

    /// Generate reports
    Report {
        /// Show report for current week
        #[arg(long)]
        week: bool,

        /// Show report for current month
        #[arg(long)]
        month: bool,

        /// Show all-time report
        #[arg(long)]
        all: bool,
    },
}

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Set hourly rate for a role
    SetRate {
        /// Role to set rate for
        role: String,

        /// Hourly rate in USD
        rate: f64,
    },

    /// List all configured rates
    ListRates,

    /// Reset all rates to defaults
    Reset,
}
