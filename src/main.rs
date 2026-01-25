mod calculator;
mod cli;
mod commands;
mod config;
mod helpers;
mod models;
mod storage;
mod time_parser;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands, ConfigCommands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Start {
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
        } => {
            commands::start::run(
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
            )?;
        }

        Commands::Log {
            start,
            end,
            duration,
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
        } => {
            commands::log::run(
                start,
                end,
                duration,
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
            )?;
        }

        Commands::Config { action } => match action {
            ConfigCommands::SetRate { role, rate } => {
                commands::config::set_rate(&role, rate)?;
            }
            ConfigCommands::ListRates => {
                commands::config::list_rates()?;
            }
            ConfigCommands::Reset => {
                commands::config::reset()?;
            }
        },

        Commands::History { limit } => {
            commands::history::run(limit)?;
        }

        Commands::Report { week, month, all } => {
            commands::report::run(week, month, all)?;
        }
    }

    Ok(())
}
