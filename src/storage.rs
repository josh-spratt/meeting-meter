use anyhow::{Context, Result};
use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;

use crate::models::Meeting;

pub struct Storage {
    data_dir: PathBuf,
    meetings_file: PathBuf,
}

impl Storage {
    pub fn new() -> Result<Self> {
        let proj_dirs = ProjectDirs::from("com", "meeting-meter", "meeting-meter")
            .context("Failed to determine data directory")?;

        let data_dir = proj_dirs.data_dir().to_path_buf();
        fs::create_dir_all(&data_dir).context("Failed to create data directory")?;

        let meetings_file = data_dir.join("meetings.json");

        Ok(Self {
            data_dir,
            meetings_file,
        })
    }

    pub fn save_meeting(&self, meeting: &Meeting) -> Result<()> {
        let mut meetings = self.load_all_meetings()?;
        meetings.push(meeting.clone());

        let contents =
            serde_json::to_string_pretty(&meetings).context("Failed to serialize meetings")?;

        fs::write(&self.meetings_file, contents).context("Failed to write meetings file")?;

        Ok(())
    }

    pub fn load_all_meetings(&self) -> Result<Vec<Meeting>> {
        if !self.meetings_file.exists() {
            return Ok(Vec::new());
        }

        let contents =
            fs::read_to_string(&self.meetings_file).context("Failed to read meetings file")?;

        let meetings: Vec<Meeting> =
            serde_json::from_str(&contents).context("Failed to parse meetings file")?;

        Ok(meetings)
    }

    pub fn get_recent_meetings(&self, limit: usize) -> Result<Vec<Meeting>> {
        let mut meetings = self.load_all_meetings()?;
        meetings.sort_by(|a, b| b.start_time.cmp(&a.start_time));
        Ok(meetings.into_iter().take(limit).collect())
    }
}
