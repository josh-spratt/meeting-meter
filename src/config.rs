use anyhow::{Context, Result};
use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;

use crate::models::{Config, Role};

pub struct ConfigManager {
    config_path: PathBuf,
}

impl ConfigManager {
    pub fn new() -> Result<Self> {
        let proj_dirs = ProjectDirs::from("com", "meeting-meter", "meeting-meter")
            .context("Failed to determine config directory")?;

        let config_dir = proj_dirs.config_dir();
        fs::create_dir_all(config_dir).context("Failed to create config directory")?;

        let config_path = config_dir.join("config.toml");

        Ok(Self { config_path })
    }

    pub fn load(&self) -> Result<Config> {
        if !self.config_path.exists() {
            // Create default config
            let config = Config::default();
            self.save(&config)?;
            return Ok(config);
        }

        let contents =
            fs::read_to_string(&self.config_path).context("Failed to read config file")?;

        let config: Config = toml::from_str(&contents).context("Failed to parse config file")?;

        Ok(config)
    }

    pub fn save(&self, config: &Config) -> Result<()> {
        let contents = toml::to_string_pretty(config).context("Failed to serialize config")?;

        fs::write(&self.config_path, contents).context("Failed to write config file")?;

        Ok(())
    }

    pub fn set_rate(&self, role: Role, rate: f64) -> Result<()> {
        let mut config = self.load()?;
        config.rates.insert(role, rate);
        self.save(&config)?;
        Ok(())
    }

    pub fn reset(&self) -> Result<()> {
        let config = Config::default();
        self.save(&config)?;
        Ok(())
    }

    pub fn config_path(&self) -> &PathBuf {
        &self.config_path
    }
}
