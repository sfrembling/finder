use std::path::{Path, PathBuf};

use indicatif::{ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

/// A cached index of all files on the system.
#[derive(Deserialize, Serialize)]
pub struct Index {
    pub files: Vec<PathBuf>,
    last_update: chrono::DateTime<chrono::Utc>,
}

impl Index {
    /// The number of days before the index is considered outdated.
    const EXPIRATION: i64 = 7;

    /// Initialize an empty index buffer.
    pub fn new() -> Self {
        Index {
            files: Vec::new(),
            last_update: chrono::Utc::now(),
        }
    }

    /// Add a file to the index.
    pub fn add(&mut self, file: PathBuf) {
        self.files.push(file);
    }

    /// Saves the index to a specified file.
    pub fn save(&self, path: &Path) -> Result<(), std::io::Error> {
        let serialized = serde_json::to_string_pretty(&self)?;
        std::fs::write(path, serialized)
    }

    /// Loads the index from a specified file.
    pub fn load(path: &Path) -> Result<Self, std::io::Error> {
        let serialized = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&serialized)?)
    }

    /// Checks if the index is outdated.
    pub fn is_outdated(&self) -> bool {
        let now = chrono::Utc::now();
        let duration = now - self.last_update;
        duration.num_days() > Self::EXPIRATION
    }

    /// Builds the index by walking the file system.
    pub fn build(&mut self) -> Result<(), std::io::Error> {
        let pb = create_spinner("Building index...");
        for entry in WalkDir::new("/")
            .into_iter()
            .flatten()
            .flat_map(|p| p.path().canonicalize())
        {
            self.add(entry);
        }

        pb.finish_and_clear();
        Ok(())
    }
}

pub fn create_spinner(msg: &str) -> ProgressBar {
    // Code acquired from: https://github.com/console-rs/indicatif/blob/main/examples/long-spinner.rs
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(std::time::Duration::from_millis(120));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.blue} {msg}")
            .unwrap()
            .tick_strings(&[
                "▹▹▹▹▹",
                "▸▹▹▹▹",
                "▹▸▹▹▹",
                "▹▹▸▹▹",
                "▹▹▹▸▹",
                "▹▹▹▹▸",
                "▪▪▪▪▪",
            ]),
    );
    pb.set_message(msg.to_owned());
    pb
}
