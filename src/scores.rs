use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreEntry {
    pub mode: String,
    pub wpm: f64,
    pub accuracy: f64,
    pub max_combo: u32,
    pub words_typed: u32,
    pub timestamp: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ScoreBoard {
    pub scores: Vec<ScoreEntry>,
}

fn scores_path() -> PathBuf {
    let mut p = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    p.push(".phosphor-typer");
    fs::create_dir_all(&p).ok();
    p.push("scores.json");
    p
}

impl ScoreBoard {
    pub fn load() -> Self {
        let path = scores_path();
        if path.exists() {
            let data = fs::read_to_string(&path).unwrap_or_default();
            serde_json::from_str(&data).unwrap_or_default()
        } else {
            Self::default()
        }
    }

    pub fn save(&self) {
        let path = scores_path();
        if let Ok(data) = serde_json::to_string_pretty(self) {
            fs::write(path, data).ok();
        }
    }

    pub fn add(&mut self, entry: ScoreEntry) {
        self.scores.push(entry);
        // Keep top 50 scores sorted by WPM descending
        self.scores.sort_by(|a, b| b.wpm.partial_cmp(&a.wpm).unwrap_or(std::cmp::Ordering::Equal));
        self.scores.truncate(50);
        self.save();
    }

    pub fn top_for_mode(&self, mode: &str, count: usize) -> Vec<&ScoreEntry> {
        self.scores
            .iter()
            .filter(|s| s.mode == mode)
            .take(count)
            .collect()
    }
}
