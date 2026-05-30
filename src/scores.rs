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

    /// Maximum number of scores retained on the board.
    const MAX_SCORES: usize = 50;

    pub fn add(&mut self, entry: ScoreEntry) {
        self.scores.push(entry);
        self.rerank();
        self.save();
    }

    /// Sort by WPM descending and keep only the top [`Self::MAX_SCORES`].
    /// Pure (no I/O) so it can be unit-tested directly.
    fn rerank(&mut self) {
        self.scores.sort_by(|a, b| {
            b.wpm
                .partial_cmp(&a.wpm)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        self.scores.truncate(Self::MAX_SCORES);
    }

    pub fn top_for_mode(&self, mode: &str, count: usize) -> Vec<&ScoreEntry> {
        self.scores
            .iter()
            .filter(|s| s.mode == mode)
            .take(count)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(mode: &str, wpm: f64) -> ScoreEntry {
        ScoreEntry {
            mode: mode.to_string(),
            wpm,
            accuracy: 100.0,
            max_combo: 0,
            words_typed: 0,
            timestamp: "0".to_string(),
        }
    }

    fn board(entries: Vec<ScoreEntry>) -> ScoreBoard {
        ScoreBoard { scores: entries }
    }

    #[test]
    fn rerank_sorts_by_wpm_descending() {
        let mut b = board(vec![
            entry("classic", 40.0),
            entry("classic", 90.0),
            entry("classic", 65.0),
        ]);
        b.rerank();
        let wpms: Vec<f64> = b.scores.iter().map(|e| e.wpm).collect();
        assert_eq!(wpms, vec![90.0, 65.0, 40.0]);
    }

    #[test]
    fn rerank_truncates_to_max_scores() {
        let many = (0..120).map(|i| entry("classic", i as f64)).collect();
        let mut b = board(many);
        b.rerank();
        assert_eq!(b.scores.len(), ScoreBoard::MAX_SCORES);
        // The highest WPM must survive truncation.
        assert_eq!(b.scores.first().unwrap().wpm, 119.0);
    }

    #[test]
    fn top_for_mode_filters_and_limits() {
        let b = board(vec![
            entry("classic", 80.0),
            entry("hacker", 70.0),
            entry("classic", 60.0),
            entry("classic", 50.0),
        ]);
        let top = b.top_for_mode("classic", 2);
        assert_eq!(top.len(), 2);
        assert!(top.iter().all(|e| e.mode == "classic"));
        assert_eq!(top[0].wpm, 80.0);
        assert_eq!(top[1].wpm, 60.0);

        assert_eq!(b.top_for_mode("nonexistent", 5).len(), 0);
    }
}
