# phosphor-typer

Terminal typing speed game with three modes and a phosphor-green cyberpunk aesthetic.

## Features

- **Classic mode**: type prompted words against a timer, tracks WPM and accuracy
- **Hacker mode**: code-style snippets with symbols and special characters
- **Cascade mode**: words fall down the screen -- type them before they hit the bottom
- Live WPM, accuracy, combo counter, and difficulty scaling
- Persistent high score board (saved to disk via `dirs`)
- Title screen, mode select menu, and game-over stats screen
- ~30 fps animation in cascade mode

## Install

```
cargo build --release
```

## Usage

```
phosphor-typer                # launch with title screen
phosphor-typer --mode classic # jump straight into classic mode
phosphor-typer --mode hacker  # jump straight into hacker mode
phosphor-typer --mode cascade # jump straight into cascade mode
```

## Keybindings

| Key         | Context     | Action                     |
|-------------|-------------|----------------------------|
| `Enter`     | Title       | Open mode select           |
| `1` / `2` / `3` | Mode select | Pick classic / hacker / cascade |
| Arrow keys  | Mode select | Navigate menu              |
| Characters  | Playing     | Type the prompted text     |
| `Backspace` | Playing     | Delete last character      |
| `Tab`       | Playing     | Skip current word          |
| `Esc`       | Playing     | Return to mode select      |
| `Enter`     | Game over   | Play again                 |
| `h`         | Title/Game over | View high scores       |
| `q`         | Any         | Quit                       |

---

Built with Rust + ratatui.
