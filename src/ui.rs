use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::game::{GameMode, GameState, Screen};

const PHOSPHOR: Color = Color::Rgb(0, 255, 0);
const PHOSPHOR_DIM: Color = Color::Rgb(0, 180, 0);
const PHOSPHOR_DARK: Color = Color::Rgb(0, 100, 0);
const ERROR_RED: Color = Color::Rgb(255, 40, 40);
const BG: Color = Color::Rgb(0, 0, 0);
const AMBER: Color = Color::Rgb(255, 176, 0);

const TITLE_ART: &str = r#"
 ██▓███   ██░ ██  ▒█████    ██████  ██▓███   ██░ ██  ▒█████   ██▀███
▓██░  ██▒▓██░ ██▒▒██▒  ██▒▒██    ▒ ▓██░  ██▒▓██░ ██▒▒██▒  ██▒▓██ ▒ ██▒
▓██░ ██▓▒▒██▀▀██░▒██░  ██▒░ ▓██▄   ▓██░ ██▓▒▒██▀▀██░▒██░  ██▒▓██ ░▄█ ▒
▒██▄█▓▒ ▒░▓█ ░██ ▒██   ██░  ▒   ██▒▒██▄█▓▒ ▒░▓█ ░██ ▒██   ██░▒██▀▀█▄
▒██▒ ░  ░░▓█▒░██▓░ ████▓▒░▒██████▒▒▒██▒ ░  ░░▓█▒░██▓░ ████▓▒░░██▓ ▒██▒
▒▓▒░ ░  ░ ▒ ░░▒░▒░ ▒░▒░▒░ ▒ ▒▓▒ ▒ ░▒▓▒░ ░  ░ ▒ ░░▒░▒░ ▒░▒░▒░ ░ ▒▓ ░▒▓░
░▒ ░      ▒ ░▒░ ░  ░ ▒ ▒░ ░ ░▒  ░ ░░▒ ░      ▒ ░▒░ ░  ░ ▒ ▒░   ░▒ ░ ▒░
░░        ░  ░░ ░░ ░ ░ ▒  ░  ░  ░  ░░        ░  ░░ ░░ ░ ░ ▒    ░░   ░
          ░  ░  ░    ░ ░        ░              ░  ░  ░    ░ ░     ░

              ▄▄▄█████▓▓██   ██▓ ██▓███  ▓█████  ██▀███
              ▓  ██▒ ▓▒ ▒██  ██▒▓██░  ██▒▓█   ▀ ▓██ ▒ ██▒
              ▒ ▓██░ ▒░  ▒██ ██░▓██░ ██▓▒▒███   ▓██ ░▄█ ▒
              ░ ▓██▓ ░   ░ ▐██▓░▒██▄█▓▒ ▒▒▓█  ▄▒██▀▀█▄
                ▒██▒ ░   ░ ██▒▓░▒██▒ ░  ░░▒████▒░██▓ ▒██▒
                ▒ ░░      ██▒▒▒ ▒▓▒░ ░  ░░░ ▒░ ░░ ▒▓ ░▒▓░
                  ░     ▓██ ░▒░ ░▒ ░      ░ ░  ░  ░▒ ░ ▒░
                ░       ▒ ▒ ░░  ░░          ░     ░░   ░
                        ░ ░                 ░  ░   ░
"#;

pub fn draw(f: &mut Frame, state: &GameState) {
    let area = f.area();

    // Full black background
    let bg_block = Block::default().style(Style::default().bg(BG));
    f.render_widget(bg_block, area);

    match state.screen {
        Screen::Title => draw_title(f, area, state),
        Screen::ModeSelect => draw_mode_select(f, area, state),
        Screen::Playing => draw_playing(f, area, state),
        Screen::GameOver => draw_game_over(f, area, state),
        Screen::HighScores => draw_high_scores(f, area, state),
    }
}

fn draw_title(f: &mut Frame, area: Rect, state: &GameState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Length(20),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(area);

    // Title ASCII art
    let title_lines: Vec<Line> = TITLE_ART
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let color = if state.scanline_offset == (i % 2) as u16 {
                PHOSPHOR
            } else {
                PHOSPHOR_DIM
            };
            Line::from(Span::styled(line, Style::default().fg(color)))
        })
        .collect();

    let title = Paragraph::new(title_lines).alignment(Alignment::Center);
    f.render_widget(title, chunks[1]);

    // Subtitle
    let subtitle = Paragraph::new(vec![
        Line::from(Span::styled(
            ">> TERMINAL TYPING INTERFACE v0.1.0 <<",
            Style::default().fg(PHOSPHOR).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "[ENTER] START  |  [H] HIGH SCORES  |  [Q] QUIT",
            Style::default().fg(PHOSPHOR_DIM),
        )),
    ])
    .alignment(Alignment::Center);
    f.render_widget(subtitle, chunks[2]);

    // Bottom bar
    let bottom = Paragraph::new(Line::from(Span::styled(
        " phosphor-typer // cyberdeck interface // all systems nominal ",
        Style::default().fg(PHOSPHOR_DARK).bg(BG),
    )))
    .alignment(Alignment::Center);
    f.render_widget(bottom, chunks[3]);
}

fn draw_mode_select(f: &mut Frame, area: Rect, state: &GameState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(2),
        ])
        .split(area);

    // Header
    let header = Paragraph::new(Line::from(Span::styled(
        ">> SELECT OPERATION MODE <<",
        Style::default().fg(PHOSPHOR).add_modifier(Modifier::BOLD),
    )))
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::BOTTOM)
            .border_style(Style::default().fg(PHOSPHOR_DARK)),
    );
    f.render_widget(header, chunks[0]);

    let modes = vec![
        (
            "CLASSIC",
            "Standard typing test. 60-second rounds.",
            "Hacker-themed words. Pure speed.",
        ),
        (
            "HACKER MODE",
            "Type real code snippets. Rust, Python, bash.",
            "Brackets, semicolons, the works.",
        ),
        (
            "CASCADE",
            "Words fall from above. Type them before impact.",
            "Survival mode. 10 misses and you're out.",
        ),
    ];

    let mode_lines: Vec<Line> = modes
        .iter()
        .enumerate()
        .flat_map(|(i, (name, desc1, desc2))| {
            let selected = i == state.selected_menu_item;
            let (marker, name_style, desc_style) = if selected {
                (
                    ">> ",
                    Style::default()
                        .fg(PHOSPHOR)
                        .add_modifier(Modifier::BOLD | Modifier::REVERSED),
                    Style::default().fg(PHOSPHOR),
                )
            } else {
                (
                    "   ",
                    Style::default().fg(PHOSPHOR_DIM),
                    Style::default().fg(PHOSPHOR_DARK),
                )
            };

            vec![
                Line::from(""),
                Line::from(vec![
                    Span::styled(marker, Style::default().fg(PHOSPHOR)),
                    Span::styled(format!("[{}] {}", i + 1, name), name_style),
                ]),
                Line::from(Span::styled(format!("      {}", desc1), desc_style)),
                Line::from(Span::styled(format!("      {}", desc2), desc_style)),
            ]
        })
        .collect();

    let modes_widget = Paragraph::new(mode_lines).alignment(Alignment::Left);
    f.render_widget(modes_widget, chunks[1]);

    let footer = Paragraph::new(Line::from(Span::styled(
        "[1/2/3] or [UP/DOWN + ENTER] to select  |  [ESC] back",
        Style::default().fg(PHOSPHOR_DARK),
    )))
    .alignment(Alignment::Center);
    f.render_widget(footer, chunks[2]);
}

fn draw_playing(f: &mut Frame, area: Rect, state: &GameState) {
    if state.mode == GameMode::Cascade {
        draw_cascade(f, area, state);
        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .split(area);

    // Stats bar
    draw_stats_bar(f, chunks[0], state);

    // Main typing area
    draw_typing_area(f, chunks[1], state);

    // Input line
    draw_input_line(f, chunks[2], state);

    // Bottom hint
    let hint = Paragraph::new(Line::from(Span::styled(
        "[TAB] skip word  |  [ESC] abort round",
        Style::default().fg(PHOSPHOR_DARK),
    )))
    .alignment(Alignment::Center);
    f.render_widget(hint, chunks[3]);
}

fn draw_stats_bar(f: &mut Frame, area: Rect, state: &GameState) {
    let time_left = state.time_remaining();
    let time_str = if state.mode == GameMode::Cascade {
        format!("TIME: {:02}:{:02}", time_left.as_secs() / 60, time_left.as_secs() % 60)
    } else {
        format!("TIME: {:02}s", time_left.as_secs())
    };

    let combo_color = if state.combo >= 10 {
        AMBER
    } else if state.combo >= 5 {
        PHOSPHOR
    } else {
        PHOSPHOR_DIM
    };

    let stats = Line::from(vec![
        Span::styled(
            format!(" WPM: {:.0} ", state.wpm),
            Style::default().fg(PHOSPHOR).add_modifier(Modifier::BOLD),
        ),
        Span::styled(" | ", Style::default().fg(PHOSPHOR_DARK)),
        Span::styled(
            format!("ACC: {:.1}% ", state.accuracy),
            Style::default().fg(if state.accuracy >= 95.0 {
                PHOSPHOR
            } else if state.accuracy >= 80.0 {
                PHOSPHOR_DIM
            } else {
                ERROR_RED
            }),
        ),
        Span::styled(" | ", Style::default().fg(PHOSPHOR_DARK)),
        Span::styled(
            format!("COMBO: {}x ", state.combo),
            Style::default().fg(combo_color).add_modifier(Modifier::BOLD),
        ),
        Span::styled(" | ", Style::default().fg(PHOSPHOR_DARK)),
        Span::styled(
            format!("WORDS: {} ", state.correct_words),
            Style::default().fg(PHOSPHOR_DIM),
        ),
        Span::styled(" | ", Style::default().fg(PHOSPHOR_DARK)),
        Span::styled(
            time_str,
            Style::default()
                .fg(if state.time_remaining().as_secs() <= 10 && state.mode != GameMode::Cascade {
                    ERROR_RED
                } else {
                    PHOSPHOR
                })
                .add_modifier(Modifier::BOLD),
        ),
    ]);

    let bar = Paragraph::new(stats).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(PHOSPHOR_DARK))
            .title(Span::styled(
                " STATS ",
                Style::default().fg(PHOSPHOR).add_modifier(Modifier::BOLD),
            )),
    );
    f.render_widget(bar, area);
}

fn draw_typing_area(f: &mut Frame, area: Rect, state: &GameState) {
    let inner = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(PHOSPHOR_DARK))
        .title(Span::styled(
            format!(" {} ", state.mode.label().to_uppercase()),
            Style::default().fg(PHOSPHOR).add_modifier(Modifier::BOLD),
        ));

    let inner_area = inner.inner(area);
    f.render_widget(inner, area);

    // Build word display
    let mut lines: Vec<Line> = Vec::new();
    let mut current_line_spans: Vec<Span> = Vec::new();
    let mut current_line_len: usize = 0;
    let max_width = inner_area.width as usize;

    // Show a window of words around the current one
    let start = state.current_word_idx.saturating_sub(2);
    let end = (state.current_word_idx + 30).min(state.words.len());

    for (idx, word) in state.words[start..end].iter().enumerate() {
        let abs_idx = start + idx;
        let word_display = format!("{} ", word);
        let word_display_len = word_display.len();

        if current_line_len + word_display_len > max_width && !current_line_spans.is_empty() {
            lines.push(Line::from(current_line_spans.clone()));
            current_line_spans.clear();
            current_line_len = 0;
        }

        if abs_idx < state.current_word_idx {
            // Already typed
            current_line_spans.push(Span::styled(
                word_display,
                Style::default().fg(PHOSPHOR_DARK),
            ));
        } else if abs_idx == state.current_word_idx {
            // Current word - show character-by-character coloring
            let chars: Vec<char> = word.chars().collect();
            let typed_chars: Vec<char> = state.typed.chars().collect();

            for (ci, ch) in chars.iter().enumerate() {
                let style = if ci < typed_chars.len() {
                    if typed_chars[ci] == *ch {
                        Style::default()
                            .fg(Color::Rgb(0, 0, 0))
                            .bg(PHOSPHOR)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default()
                            .fg(Color::Rgb(255, 255, 255))
                            .bg(ERROR_RED)
                            .add_modifier(Modifier::BOLD)
                    }
                } else if ci == typed_chars.len() {
                    // Cursor position
                    Style::default()
                        .fg(PHOSPHOR)
                        .add_modifier(Modifier::UNDERLINED | Modifier::BOLD)
                } else {
                    Style::default().fg(PHOSPHOR).add_modifier(Modifier::BOLD)
                };
                current_line_spans.push(Span::styled(ch.to_string(), style));
            }

            // Extra typed chars beyond word length
            if typed_chars.len() > chars.len() {
                for tc in &typed_chars[chars.len()..] {
                    current_line_spans.push(Span::styled(
                        tc.to_string(),
                        Style::default()
                            .fg(Color::Rgb(255, 255, 255))
                            .bg(ERROR_RED),
                    ));
                }
            }

            current_line_spans.push(Span::styled(" ", Style::default()));
        } else {
            // Upcoming words
            let row_idx = lines.len();
            let color = if state.scanline_offset == (row_idx % 2) as u16 {
                PHOSPHOR_DIM
            } else {
                PHOSPHOR_DARK
            };
            current_line_spans.push(Span::styled(word_display, Style::default().fg(color)));
        }

        current_line_len += word_display_len;
    }

    if !current_line_spans.is_empty() {
        lines.push(Line::from(current_line_spans));
    }

    let para = Paragraph::new(lines).wrap(Wrap { trim: false });
    f.render_widget(para, inner_area);
}

fn draw_input_line(f: &mut Frame, area: Rect, state: &GameState) {
    let input_text = if state.mode == GameMode::Cascade {
        &state.cascade_typed
    } else {
        &state.typed
    };

    let input = Paragraph::new(Line::from(vec![
        Span::styled(
            " > ",
            Style::default().fg(PHOSPHOR).add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            input_text.as_str(),
            Style::default().fg(PHOSPHOR).add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            "█",
            Style::default().fg(PHOSPHOR).add_modifier(Modifier::SLOW_BLINK),
        ),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(PHOSPHOR_DARK))
            .title(Span::styled(
                " INPUT ",
                Style::default().fg(PHOSPHOR).add_modifier(Modifier::BOLD),
            )),
    );
    f.render_widget(input, area);
}

fn draw_cascade(f: &mut Frame, area: Rect, state: &GameState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(area);

    // Stats bar (reuse)
    let cascade_stats = Line::from(vec![
        Span::styled(
            format!(" SCORE: {} ", state.cascade_score),
            Style::default().fg(AMBER).add_modifier(Modifier::BOLD),
        ),
        Span::styled(" | ", Style::default().fg(PHOSPHOR_DARK)),
        Span::styled(
            format!("COMBO: {}x ", state.combo),
            Style::default()
                .fg(if state.combo >= 5 { AMBER } else { PHOSPHOR_DIM })
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" | ", Style::default().fg(PHOSPHOR_DARK)),
        Span::styled(
            format!("MISSED: {}/10 ", state.cascade_missed),
            Style::default().fg(if state.cascade_missed >= 7 {
                ERROR_RED
            } else {
                PHOSPHOR_DIM
            }),
        ),
        Span::styled(" | ", Style::default().fg(PHOSPHOR_DARK)),
        Span::styled(
            format!("WPM: {:.0} ", state.wpm),
            Style::default().fg(PHOSPHOR),
        ),
        Span::styled(" | ", Style::default().fg(PHOSPHOR_DARK)),
        Span::styled(
            format!(
                "TIME: {:02}:{:02}",
                state.elapsed.as_secs() / 60,
                state.elapsed.as_secs() % 60
            ),
            Style::default().fg(PHOSPHOR),
        ),
    ]);

    let stats_bar = Paragraph::new(cascade_stats).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(PHOSPHOR_DARK))
            .title(Span::styled(
                " CASCADE ",
                Style::default().fg(AMBER).add_modifier(Modifier::BOLD),
            )),
    );
    f.render_widget(stats_bar, chunks[0]);

    // Cascade field
    let field_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(PHOSPHOR_DARK));
    let field_inner = field_block.inner(chunks[1]);
    f.render_widget(field_block, chunks[1]);

    // Draw falling words
    for fw in &state.falling_words {
        let row = fw.row_f as u16;
        if row < field_inner.height {
            let x = fw.col.min(field_inner.width.saturating_sub(fw.word.len() as u16));
            let render_area = Rect::new(
                field_inner.x + x,
                field_inner.y + row,
                fw.word.len() as u16,
                1,
            );

            // Highlight if matching current typed prefix
            let is_match = !state.cascade_typed.is_empty()
                && fw.word.starts_with(state.cascade_typed.as_str());

            let word_spans: Vec<Span> = if is_match {
                let typed_len = state.cascade_typed.len();
                fw.word
                    .chars()
                    .enumerate()
                    .map(|(i, c)| {
                        if i < typed_len {
                            Span::styled(
                                c.to_string(),
                                Style::default()
                                    .fg(Color::Rgb(0, 0, 0))
                                    .bg(PHOSPHOR)
                                    .add_modifier(Modifier::BOLD),
                            )
                        } else {
                            Span::styled(
                                c.to_string(),
                                Style::default().fg(PHOSPHOR).add_modifier(Modifier::BOLD),
                            )
                        }
                    })
                    .collect()
            } else {
                // Color based on vertical position (urgency)
                let urgency = row as f64 / field_inner.height as f64;
                let color = if urgency > 0.8 {
                    ERROR_RED
                } else if urgency > 0.6 {
                    AMBER
                } else {
                    PHOSPHOR
                };
                vec![Span::styled(
                    fw.word.clone(),
                    Style::default().fg(color),
                )]
            };

            let word_para = Paragraph::new(Line::from(word_spans));
            f.render_widget(word_para, render_area);
        }
    }

    // Danger zone line at bottom
    if field_inner.height > 2 {
        let danger_y = field_inner.y + field_inner.height - 1;
        let danger_line = "─".repeat(field_inner.width as usize);
        let danger = Paragraph::new(Line::from(Span::styled(
            danger_line,
            Style::default().fg(ERROR_RED).add_modifier(Modifier::DIM),
        )));
        f.render_widget(
            danger,
            Rect::new(field_inner.x, danger_y, field_inner.width, 1),
        );
    }

    // Input line
    draw_input_line(f, chunks[2], state);
}

fn draw_game_over(f: &mut Frame, area: Rect, state: &GameState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Length(5),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(area);

    // Header
    let header = Paragraph::new(Line::from(Span::styled(
        ">> ROUND COMPLETE <<",
        Style::default().fg(PHOSPHOR).add_modifier(Modifier::BOLD),
    )))
    .alignment(Alignment::Center);
    f.render_widget(header, chunks[0]);

    // Big WPM display
    let wpm_display = format!("{:.0}", state.wpm);
    let big_wpm = Paragraph::new(vec![
        Line::from(Span::styled(
            format!("  {}  WPM", wpm_display),
            Style::default()
                .fg(PHOSPHOR)
                .add_modifier(Modifier::BOLD),
        )),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(PHOSPHOR_DIM))
            .title(Span::styled(
                " RESULTS ",
                Style::default().fg(PHOSPHOR).add_modifier(Modifier::BOLD),
            )),
    );
    f.render_widget(big_wpm, chunks[1]);

    // Detailed stats
    let mut stats_lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("  Mode:       ", Style::default().fg(PHOSPHOR_DIM)),
            Span::styled(
                state.mode.label().to_uppercase(),
                Style::default().fg(PHOSPHOR).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("  WPM:        ", Style::default().fg(PHOSPHOR_DIM)),
            Span::styled(
                format!("{:.1}", state.wpm),
                Style::default().fg(PHOSPHOR).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("  Accuracy:   ", Style::default().fg(PHOSPHOR_DIM)),
            Span::styled(
                format!("{:.1}%", state.accuracy),
                Style::default().fg(if state.accuracy >= 95.0 {
                    PHOSPHOR
                } else if state.accuracy >= 80.0 {
                    PHOSPHOR_DIM
                } else {
                    ERROR_RED
                }),
            ),
        ]),
        Line::from(vec![
            Span::styled("  Words:      ", Style::default().fg(PHOSPHOR_DIM)),
            Span::styled(
                format!("{} / {}", state.correct_words, state.total_words_attempted),
                Style::default().fg(PHOSPHOR),
            ),
        ]),
        Line::from(vec![
            Span::styled("  Max Combo:  ", Style::default().fg(PHOSPHOR_DIM)),
            Span::styled(
                format!("{}x", state.max_combo),
                Style::default().fg(AMBER),
            ),
        ]),
    ];

    if state.mode == GameMode::Cascade {
        stats_lines.push(Line::from(vec![
            Span::styled("  Score:      ", Style::default().fg(PHOSPHOR_DIM)),
            Span::styled(
                format!("{}", state.cascade_score),
                Style::default().fg(AMBER).add_modifier(Modifier::BOLD),
            ),
        ]));
        stats_lines.push(Line::from(vec![
            Span::styled("  Missed:     ", Style::default().fg(PHOSPHOR_DIM)),
            Span::styled(
                format!("{}", state.cascade_missed),
                Style::default().fg(ERROR_RED),
            ),
        ]));
    }

    // Rating
    let rating = if state.wpm >= 100.0 {
        ">>> ELITE HACKER <<<"
    } else if state.wpm >= 80.0 {
        ">> SENIOR OPERATOR <<"
    } else if state.wpm >= 60.0 {
        "> FIELD AGENT <"
    } else if state.wpm >= 40.0 {
        "RECRUIT"
    } else {
        "SCRIPT KIDDIE"
    };

    stats_lines.push(Line::from(""));
    stats_lines.push(Line::from(Span::styled(
        format!("  Rank: {}", rating),
        Style::default().fg(AMBER).add_modifier(Modifier::BOLD),
    )));

    let stats = Paragraph::new(stats_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(PHOSPHOR_DARK))
            .title(Span::styled(
                " DEBRIEF ",
                Style::default().fg(PHOSPHOR).add_modifier(Modifier::BOLD),
            )),
    );
    f.render_widget(stats, chunks[2]);

    let footer = Paragraph::new(Line::from(Span::styled(
        "[ENTER] play again  |  [H] high scores  |  [ESC] menu  |  [Q] quit",
        Style::default().fg(PHOSPHOR_DIM),
    )))
    .alignment(Alignment::Center);
    f.render_widget(footer, chunks[3]);
}

fn draw_high_scores(f: &mut Frame, area: Rect, state: &GameState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(2),
        ])
        .split(area);

    let header = Paragraph::new(Line::from(Span::styled(
        ">> HIGH SCORES <<",
        Style::default().fg(PHOSPHOR).add_modifier(Modifier::BOLD),
    )))
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::BOTTOM)
            .border_style(Style::default().fg(PHOSPHOR_DARK)),
    );
    f.render_widget(header, chunks[0]);

    let mut lines: Vec<Line> = Vec::new();

    for mode in &["classic", "hacker", "cascade"] {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            format!("  ── {} ──", mode.to_uppercase()),
            Style::default().fg(AMBER).add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(""));

        let top = state.scoreboard.top_for_mode(mode, 5);
        if top.is_empty() {
            lines.push(Line::from(Span::styled(
                "    No scores recorded.",
                Style::default().fg(PHOSPHOR_DARK),
            )));
        } else {
            // Header row
            lines.push(Line::from(vec![
                Span::styled(
                    "    #  WPM     ACC     COMBO  WORDS",
                    Style::default()
                        .fg(PHOSPHOR_DIM)
                        .add_modifier(Modifier::UNDERLINED),
                ),
            ]));

            for (i, score) in top.iter().enumerate() {
                let color = if i == 0 { AMBER } else { PHOSPHOR_DIM };
                lines.push(Line::from(Span::styled(
                    format!(
                        "    {}  {:<7.1} {:<7.1} {:<6} {}",
                        i + 1,
                        score.wpm,
                        score.accuracy,
                        score.max_combo,
                        score.words_typed
                    ),
                    Style::default().fg(color),
                )));
            }
        }
    }

    let scores = Paragraph::new(lines);
    f.render_widget(scores, chunks[1]);

    let footer = Paragraph::new(Line::from(Span::styled(
        "[ESC] back",
        Style::default().fg(PHOSPHOR_DIM),
    )))
    .alignment(Alignment::Center);
    f.render_widget(footer, chunks[2]);
}
