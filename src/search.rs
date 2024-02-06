use colored::Colorize;

use crate::directory;

use super::index;
use std::{
    error::Error,
    path::{Path, PathBuf},
};

pub struct SearchResults {
    pub results: Vec<PathBuf>,
}

impl SearchResults {
    fn new() -> Self {
        SearchResults {
            results: Vec::new(),
        }
    }

    pub fn search(query: &str) -> Result<Self, Box<dyn Error>> {
        let index_path = directory::get_data_dir().join("index.json");

        if !index_path.exists() {
            return Err("Index does not exist. Please run `find --index` first.".into());
        }

        let regex = regex::Regex::new(query)?;
        let index = index::Index::load(&index_path)?;
        let mut search = SearchResults::new();

        if index.is_outdated() {
            println!(
                "{}: The index is outdated. You might not get accurate results.",
                "Warning".yellow()
            );
        }
        let pb = index::create_spinner("Searching...");
        for file in index.files {
            if regex.is_match(&file.to_string_lossy()) {
                let file = file.canonicalize();
                if let Ok(file) = file {
                    search.results.push(file);
                }
            }
        }
        pb.finish_and_clear();
        Ok(search)
    }

    /// Searches for a file under a specific path.
    pub fn search_path(query: &str, path: &str) -> Result<Self, Box<dyn Error>> {
        let mut search = Self::search(query)?;

        let path = Path::new(path).canonicalize()?;

        search.results.retain(|p| p.starts_with(&path));

        Ok(search)
    }

    pub fn display(&self) {
        let buffer = self.format();

        println!("{}", buffer.trim());
    }

    pub fn format(&self) -> String {
        let results = &self.results;
        let mut buffer = String::new();
        for result in results
            .iter()
            .map(|p| p.display().to_string())
            .map(|s| s.trim_start_matches("\\\\?\\").to_owned())
        {
            buffer.push_str(&format!("{}\n", result));
        }

        buffer.trim().to_string()
    }

    pub fn save(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let buffer = self.format();
        Ok(std::fs::write(path, buffer)?)
    }
}
