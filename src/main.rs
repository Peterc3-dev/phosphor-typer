mod game;
mod scores;
mod ui;
mod words;

use std::io;
use std::time::Duration;

use clap::Parser;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use game::{GameMode, GameState, Screen};

#[derive(Parser)]
#[command(name = "phosphor-typer")]
#[command(about = "Cyberpunk terminal typing speed game")]
#[command(version)]
struct Cli {
    /// Game mode: classic, hacker, cascade
    #[arg(short, long, default_value = "menu")]
    mode: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Restore terminal on panic so the user isn't left in raw mode
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = crossterm::terminal::disable_raw_mode();
        let _ = crossterm::execute!(std::io::stdout(), crossterm::terminal::LeaveAlternateScreen);
        original_hook(info);
    }));

    // Terminal setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_app(&mut terminal, &cli.mode);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(e) = result {
        eprintln!("Error: {}", e);
    }

    Ok(())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    initial_mode: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut state = GameState::new();

    // If a mode was specified on CLI, jump straight to playing
    match initial_mode {
        "classic" => {
            state.mode = GameMode::Classic;
            state.start_round();
        }
        "hacker" => {
            state.mode = GameMode::Hacker;
            state.start_round();
        }
        "cascade" => {
            state.mode = GameMode::Cascade;
            state.start_round();
        }
        _ => {
            // Show title screen
        }
    }

    loop {
        // Update cascade dimensions from terminal size
        let size = terminal.size()?;
        state.cascade_height = size.height;
        state.cascade_width = size.width;

        // Tick game state
        if state.screen == Screen::Playing {
            state.tick();
            // Re-check screen after tick (might have ended the round)
            if state.screen != Screen::Playing {
                terminal.draw(|f| ui::draw(f, &state))?;
                continue;
            }
        }

        terminal.draw(|f| ui::draw(f, &state))?;

        // Poll for events with a short timeout for smooth animation
        let timeout = if state.screen == Screen::Playing {
            Duration::from_millis(33) // ~30 fps
        } else {
            Duration::from_millis(100)
        };

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                // Only handle key press events (not release/repeat on some platforms)
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                match state.screen {
                    Screen::Title => match key.code {
                        KeyCode::Enter => {
                            state.screen = Screen::ModeSelect;
                            state.selected_menu_item = 0;
                        }
                        KeyCode::Char('h') | KeyCode::Char('H') => {
                            state.screen = Screen::HighScores;
                        }
                        KeyCode::Char('q') | KeyCode::Char('Q') => return Ok(()),
                        KeyCode::Esc => return Ok(()),
                        _ => {}
                    },

                    Screen::ModeSelect => match key.code {
                        KeyCode::Char('1') => {
                            state.mode = GameMode::Classic;
                            state.start_round();
                        }
                        KeyCode::Char('2') => {
                            state.mode = GameMode::Hacker;
                            state.start_round();
                        }
                        KeyCode::Char('3') => {
                            state.mode = GameMode::Cascade;
                            state.start_round();
                        }
                        KeyCode::Up if state.selected_menu_item > 0 => {
                            state.selected_menu_item -= 1;
                        }
                        KeyCode::Down if state.selected_menu_item < 2 => {
                            state.selected_menu_item += 1;
                        }
                        KeyCode::Enter => {
                            state.mode = match state.selected_menu_item {
                                0 => GameMode::Classic,
                                1 => GameMode::Hacker,
                                _ => GameMode::Cascade,
                            };
                            state.start_round();
                        }
                        KeyCode::Esc => {
                            state.screen = Screen::Title;
                        }
                        KeyCode::Char('q') | KeyCode::Char('Q') => return Ok(()),
                        _ => {}
                    },

                    Screen::Playing => match key.code {
                        KeyCode::Esc => {
                            state.screen = Screen::ModeSelect;
                        }
                        KeyCode::Backspace => {
                            state.handle_backspace();
                        }
                        KeyCode::Tab => {
                            state.handle_space_skip();
                        }
                        KeyCode::Char(c) => {
                            state.handle_char(c);
                        }
                        _ => {}
                    },

                    Screen::GameOver => match key.code {
                        KeyCode::Enter => {
                            state.start_round();
                        }
                        KeyCode::Char('h') | KeyCode::Char('H') => {
                            state.screen = Screen::HighScores;
                        }
                        KeyCode::Esc => {
                            state.screen = Screen::ModeSelect;
                        }
                        KeyCode::Char('q') | KeyCode::Char('Q') => return Ok(()),
                        _ => {}
                    },

                    Screen::HighScores => match key.code {
                        KeyCode::Esc => {
                            state.screen = Screen::Title;
                        }
                        KeyCode::Char('q') | KeyCode::Char('Q') => return Ok(()),
                        _ => {}
                    },
                }
            }
        }
    }
}
