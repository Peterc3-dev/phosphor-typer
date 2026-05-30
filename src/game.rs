use std::time::{Duration, Instant};

use crate::scores::{ScoreBoard, ScoreEntry};
use crate::words;

/// Falling word in cascade mode
#[derive(Clone, Debug)]
pub struct FallingWord {
    pub word: String,
    pub col: u16,
    pub row_f: f64, // fractional row for smooth movement
    pub speed: f64, // rows per tick
}

#[derive(Clone, Debug, PartialEq)]
pub enum GameMode {
    Classic,
    Hacker,
    Cascade,
}

impl GameMode {
    pub fn label(&self) -> &str {
        match self {
            GameMode::Classic => "classic",
            GameMode::Hacker => "hacker",
            GameMode::Cascade => "cascade",
        }
    }

    pub fn word_source(&self) -> &str {
        match self {
            GameMode::Classic => "hacker",
            GameMode::Hacker => "code",
            GameMode::Cascade => "hacker",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Screen {
    Title,
    ModeSelect,
    Playing,
    GameOver,
    HighScores,
}

pub struct GameState {
    pub screen: Screen,
    pub mode: GameMode,

    // Word management
    pub words: Vec<String>,
    pub current_word_idx: usize,
    pub typed: String,

    // Cascade mode
    pub falling_words: Vec<FallingWord>,
    pub cascade_typed: String,
    pub cascade_height: u16,
    pub cascade_width: u16,

    // Timing
    pub round_duration: Duration,
    pub start_time: Option<Instant>,
    pub elapsed: Duration,

    // Stats
    pub correct_chars: u32,
    pub total_chars: u32,
    pub correct_words: u32,
    pub total_words_attempted: u32,
    pub combo: u32,
    pub max_combo: u32,
    pub wpm: f64,
    pub accuracy: f64,
    pub difficulty: u32,

    // Cascade stats
    pub cascade_missed: u32,
    pub cascade_score: u32,

    // Visual
    pub scanline_offset: u16,
    pub tick_count: u64,

    // Scores
    pub scoreboard: ScoreBoard,
    pub selected_menu_item: usize,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            screen: Screen::Title,
            mode: GameMode::Classic,
            words: Vec::new(),
            current_word_idx: 0,
            typed: String::new(),
            falling_words: Vec::new(),
            cascade_typed: String::new(),
            cascade_height: 24,
            cascade_width: 80,
            round_duration: Duration::from_secs(60),
            start_time: None,
            elapsed: Duration::ZERO,
            correct_chars: 0,
            total_chars: 0,
            correct_words: 0,
            total_words_attempted: 0,
            combo: 0,
            max_combo: 0,
            wpm: 0.0,
            accuracy: 100.0,
            difficulty: 0,
            cascade_missed: 0,
            cascade_score: 0,
            scanline_offset: 0,
            tick_count: 0,
            scoreboard: ScoreBoard::load(),
            selected_menu_item: 0,
        }
    }

    pub fn start_round(&mut self) {
        self.words = words::get_words_for_round(self.mode.word_source(), self.difficulty);
        self.current_word_idx = 0;
        self.typed.clear();
        self.falling_words.clear();
        self.cascade_typed.clear();
        self.start_time = Some(Instant::now());
        self.elapsed = Duration::ZERO;
        self.correct_chars = 0;
        self.total_chars = 0;
        self.correct_words = 0;
        self.total_words_attempted = 0;
        self.combo = 0;
        self.max_combo = 0;
        self.wpm = 0.0;
        self.accuracy = 100.0;
        self.difficulty = 0;
        self.cascade_missed = 0;
        self.cascade_score = 0;
        self.screen = Screen::Playing;
    }

    pub fn current_word(&self) -> Option<&str> {
        self.words.get(self.current_word_idx).map(|s| s.as_str())
    }

    pub fn tick(&mut self) {
        self.tick_count = self.tick_count.wrapping_add(1);
        self.scanline_offset = (self.tick_count % 2) as u16;

        if let Some(start) = self.start_time {
            self.elapsed = start.elapsed();

            // Update WPM
            let minutes = self.elapsed.as_secs_f64() / 60.0;
            if minutes > 0.0 {
                self.wpm = self.correct_words as f64 / minutes;
            }

            // Update accuracy
            if self.total_chars > 0 {
                self.accuracy = (self.correct_chars as f64 / self.total_chars as f64) * 100.0;
            }

            // Check round end
            if self.mode != GameMode::Cascade && self.elapsed >= self.round_duration {
                self.end_round();
            }

            // Cascade mode ticking
            if self.mode == GameMode::Cascade && self.screen == Screen::Playing {
                self.tick_cascade();
            }
        }
    }

    fn tick_cascade(&mut self) {
        // Spawn new words periodically
        let spawn_interval = match self.difficulty {
            0..=1 => 30,
            2..=3 => 22,
            4..=5 => 16,
            _ => 10,
        };

        if self.tick_count.is_multiple_of(spawn_interval) {
            let word = words::get_cascade_word(self.difficulty);
            let mut rng = rand::thread_rng();
            use rand::Rng;
            let max_col = self.cascade_width.saturating_sub(word.len() as u16 + 2);
            let col = rng.gen_range(1..=max_col.max(1));
            let speed = 0.15 + (self.difficulty as f64 * 0.05);
            self.falling_words.push(FallingWord {
                word,
                col,
                row_f: 0.0,
                speed,
            });
        }

        // Move words down
        for fw in &mut self.falling_words {
            fw.row_f += fw.speed;
        }

        // Check for words that hit the bottom
        let height = self.cascade_height;
        let before = self.falling_words.len();
        self.falling_words
            .retain(|fw| (fw.row_f as u16) < height - 2);
        let removed = before - self.falling_words.len();
        self.cascade_missed += removed as u32;

        if removed > 0 {
            self.combo = 0;
        }

        // Increase difficulty over time
        self.difficulty = (self.elapsed.as_secs() / 15) as u32;

        // End cascade if too many missed
        if self.cascade_missed >= 10 {
            self.end_round();
        }
    }

    pub fn handle_char(&mut self, c: char) {
        if self.mode == GameMode::Cascade {
            self.handle_cascade_char(c);
            return;
        }

        if let Some(target) = self.current_word() {
            let target = target.to_string();
            let target_chars: Vec<char> = target.chars().collect();
            self.typed.push(c);
            self.total_chars += 1;

            let idx = self.typed.chars().count() - 1;
            if idx < target_chars.len() && target_chars[idx] == c {
                self.correct_chars += 1;
            }

            // Check if word is complete
            if self.typed.chars().count() >= target_chars.len() {
                self.total_words_attempted += 1;
                if self.typed == target {
                    self.correct_words += 1;
                    self.combo += 1;
                    if self.combo > self.max_combo {
                        self.max_combo = self.combo;
                    }
                } else {
                    self.combo = 0;
                }
                self.typed.clear();
                self.current_word_idx += 1;

                // Update difficulty based on correct words
                self.difficulty = self.correct_words / 5;

                // Generate more words if running low
                if self.current_word_idx >= self.words.len().saturating_sub(5) {
                    let mut more =
                        words::get_words_for_round(self.mode.word_source(), self.difficulty);
                    self.words.append(&mut more);
                }
            }
        }
    }

    fn handle_cascade_char(&mut self, c: char) {
        self.cascade_typed.push(c);
        self.total_chars += 1;

        // Check if any falling word matches the typed string
        let typed = &self.cascade_typed;
        if let Some(idx) = self.falling_words.iter().position(|fw| fw.word == *typed) {
            // Word matched!
            self.falling_words.remove(idx);
            self.correct_words += 1;
            self.correct_chars += typed.chars().count() as u32;
            self.combo += 1;
            if self.combo > self.max_combo {
                self.max_combo = self.combo;
            }
            self.cascade_score += (typed.chars().count() as u32) * (self.combo + 1);
            self.total_words_attempted += 1;
            self.cascade_typed.clear();
        } else {
            // Check if typed string is a prefix of any falling word
            let is_prefix = self
                .falling_words
                .iter()
                .any(|fw| fw.word.starts_with(typed.as_str()));
            if !is_prefix {
                // No match possible, reset
                self.cascade_typed.clear();
                self.combo = 0;
            }
        }
    }

    pub fn handle_backspace(&mut self) {
        if self.mode == GameMode::Cascade {
            self.cascade_typed.pop();
        } else {
            self.typed.pop();
        }
    }

    pub fn handle_space_skip(&mut self) {
        if self.mode != GameMode::Cascade {
            // Space skips to next word (counts as attempted but not correct)
            self.total_words_attempted += 1;
            self.combo = 0;
            self.typed.clear();
            self.current_word_idx += 1;
        }
    }

    pub fn time_remaining(&self) -> Duration {
        if self.mode == GameMode::Cascade {
            return self.elapsed; // Cascade shows elapsed, not remaining
        }
        self.round_duration.saturating_sub(self.elapsed)
    }

    fn end_round(&mut self) {
        self.screen = Screen::GameOver;

        let now = chrono_lite_timestamp();
        let entry = ScoreEntry {
            mode: self.mode.label().to_string(),
            wpm: self.wpm,
            accuracy: self.accuracy,
            max_combo: self.max_combo,
            words_typed: self.correct_words,
            timestamp: now,
        };
        self.scoreboard.add(entry);
    }
}

/// Simple timestamp without pulling in chrono
fn chrono_lite_timestamp() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("{}", secs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mode_label_and_source_mapping() {
        assert_eq!(GameMode::Classic.label(), "classic");
        assert_eq!(GameMode::Hacker.label(), "hacker");
        assert_eq!(GameMode::Cascade.label(), "cascade");

        // Classic plays single hacker words; Hacker mode plays code snippets.
        assert_eq!(GameMode::Classic.word_source(), "hacker");
        assert_eq!(GameMode::Hacker.word_source(), "code");
        assert_eq!(GameMode::Cascade.word_source(), "hacker");
    }

    /// Build a state with a fixed word list, no timer running, so handler
    /// logic can be exercised without touching the terminal or the clock.
    fn state_with_words(words: &[&str]) -> GameState {
        let mut state = GameState::new();
        state.mode = GameMode::Classic;
        state.words = words.iter().map(|s| s.to_string()).collect();
        state.current_word_idx = 0;
        state
    }

    fn type_str(state: &mut GameState, s: &str) {
        for c in s.chars() {
            state.handle_char(c);
        }
    }

    #[test]
    fn correct_word_advances_and_builds_combo() {
        let mut state = state_with_words(&["abc", "de"]);
        type_str(&mut state, "abc");

        assert_eq!(state.correct_words, 1);
        assert_eq!(state.combo, 1);
        assert_eq!(state.max_combo, 1);
        assert_eq!(state.total_words_attempted, 1);
        assert_eq!(state.current_word_idx, 1);
        assert_eq!(state.correct_chars, 3);
        assert_eq!(state.total_chars, 3);
        assert!(state.typed.is_empty());
    }

    #[test]
    fn wrong_word_resets_combo_but_counts_attempt() {
        let mut state = state_with_words(&["abc", "de"]);
        // Build a combo first.
        type_str(&mut state, "abc");
        assert_eq!(state.combo, 1);

        // Now mistype the next word ("de" -> "dx").
        type_str(&mut state, "dx");
        assert_eq!(state.combo, 0);
        assert_eq!(state.max_combo, 1, "max_combo should retain its peak");
        assert_eq!(state.correct_words, 1);
        assert_eq!(state.total_words_attempted, 2);
        assert_eq!(state.current_word_idx, 2);
    }

    #[test]
    fn space_skip_advances_and_breaks_combo() {
        let mut state = state_with_words(&["abc", "de"]);
        type_str(&mut state, "abc");
        assert_eq!(state.combo, 1);

        state.typed.push('d');
        state.handle_space_skip();

        assert_eq!(state.combo, 0);
        assert_eq!(state.current_word_idx, 2);
        assert_eq!(state.total_words_attempted, 2);
        assert!(state.typed.is_empty());
    }

    #[test]
    fn backspace_removes_last_typed_char() {
        let mut state = state_with_words(&["abcd"]);
        type_str(&mut state, "ab");
        state.handle_backspace();
        assert_eq!(state.typed, "a");

        // Cascade backspace operates on the cascade buffer instead.
        let mut cas = GameState::new();
        cas.mode = GameMode::Cascade;
        cas.cascade_typed = "xy".to_string();
        cas.handle_backspace();
        assert_eq!(cas.cascade_typed, "x");
    }

    #[test]
    fn cascade_match_clears_word_and_scores() {
        let mut state = GameState::new();
        state.mode = GameMode::Cascade;
        state.falling_words.push(FallingWord {
            word: "kernel".to_string(),
            col: 1,
            row_f: 0.0,
            speed: 0.1,
        });

        for c in "kernel".chars() {
            state.handle_char(c);
        }

        assert!(state.falling_words.is_empty(), "matched word removed");
        assert_eq!(state.correct_words, 1);
        assert_eq!(state.combo, 1);
        assert!(state.cascade_score > 0);
        assert!(state.cascade_typed.is_empty());
    }

    #[test]
    fn cascade_non_prefix_resets_buffer_and_combo() {
        let mut state = GameState::new();
        state.mode = GameMode::Cascade;
        state.combo = 3;
        state.falling_words.push(FallingWord {
            word: "kernel".to_string(),
            col: 1,
            row_f: 0.0,
            speed: 0.1,
        });

        // 'z' is not a prefix of "kernel": buffer and combo reset.
        state.handle_char('z');
        assert!(state.cascade_typed.is_empty());
        assert_eq!(state.combo, 0);

        // A valid prefix is retained.
        state.combo = 2;
        state.handle_char('k');
        assert_eq!(state.cascade_typed, "k");
        assert_eq!(state.combo, 2, "valid prefix keeps the combo");
    }

    #[test]
    fn time_remaining_counts_down_classic_and_up_cascade() {
        let mut state = GameState::new();
        state.mode = GameMode::Classic;
        state.round_duration = Duration::from_secs(60);
        state.elapsed = Duration::from_secs(20);
        assert_eq!(state.time_remaining(), Duration::from_secs(40));

        // Past the round length saturates at zero rather than underflowing.
        state.elapsed = Duration::from_secs(75);
        assert_eq!(state.time_remaining(), Duration::ZERO);

        // Cascade reports elapsed instead of remaining.
        state.mode = GameMode::Cascade;
        state.elapsed = Duration::from_secs(12);
        assert_eq!(state.time_remaining(), Duration::from_secs(12));
    }
}
