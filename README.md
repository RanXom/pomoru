<div align="center">

# pomoru

A minimalist Pomodoro TUI with a task list, written in Rust.

![Rust](https://img.shields.io/badge/Rust-Language-orange?logo=rust)
![ratatui](https://img.shields.io/badge/ratatui-TUI-blue)
![Tokio](https://img.shields.io/badge/Tokio-Async_Runtime-green)
![AUR](https://img.shields.io/aur/version/pomoru?logo=arch-linux)
![License](https://img.shields.io/badge/License-MIT-blue)

</div>

## Features

### Timer

- Work, short break, and long break sessions
- Customizable durations
- Desktop notifications when sessions end
- Optional automatic session switching
- Manual session progression mode

### Tasks

- Add, edit, delete tasks
- Toggle completion
- Task list persists across runs

### Design

- Keyboard-only interaction
- Minimalist interface
- Dynamic theming system (for Noctalia Shell users)
- Configuration saved locally

---

## Demo

![Demo](screenshots/demo2.gif)

---

## Keybindings

### Timer screen

| Key     | Action                             |
| ------- | ---------------------------------- |
| `space` | Start or pause timer               |
| `tab`   | Change session type (when paused)  |
| `a`     | Toggle automatic session switching |
| `e`     | Edit current session time          |
| `r`     | Reset timer                        |
| `t`     | Open task list                     |
| `q`     | Quit                               |

### Task screen

| Key         | Action               |
| ----------- | -------------------- |
| `i`         | Add new task         |
| `enter`     | Toggle task done     |
| `e`         | Edit selected task   |
| `d`         | Delete selected task |
| `j` / `↓`   | Move down            |
| `J`         | Move task down       |
| `k` / `↑`   | Move up              |
| `K`         | Move task up         |
| `t` / `esc` | Back to timer        |

---

## Configuration

`pomoru` saves its state automatically when you quit.

Config file location:

```text
~/.config/pomoru/config.toml
```

Saved data includes:

- Work and break durations
- Task list

Example:

```toml
work_time_mins = 25
short_break_mins = 5
long_break_mins = 15
auto_switch_sessions = true

[[tasks]]
title = "Read documentation"
is_done = false
```

---

## Status Export

Pomoru exports its current state to:

```text
~/.cache/pomoru/status.json
```

Example:

```json
{
  "text": "󰄉 Work 24:31",
  "tooltip": "Work",
  "class": "work"
}
```

This can be used by:

- Waybar
- Custom status bars
- Shell scripts
- Desktop widgets

---

### Waybar

The previously stated `status.json` can be used in Waybar as follows:

```json
"custom/pomoru": {
  "exec": "cat ~/.cache/pomoru/status.json",
  "return-type": "json",
  "interval": 1,
  "format": "{text}",
  "tooltip": true
}
```

---

## Installation

### Prebuilt binary

Download from the releases page:

https://github.com/RanXom/pomoru/releases

```bash
tar -xvf pomoru-1.0.0-x86_64.tar.gz
cd pomoru-1.0.0-x86_64
./pomoru
```

To install globally:

```bash
cd pomoru-1.0.0-x86_64
sudo cp pomoru /usr/local/bin/
pomoru
```

### Arch Linux (AUR)

```bash
yay -S pomoru        # build from source
yay -S pomoru-bin    # prebuilt binary
```

### Build from source

```bash
git clone https://github.com/RanXom/pomoru
cd pomoru
cargo run --release
```

To install the binary locally:

```bash
cargo install --path .
```

---

## Built with

- Rust
- ratatui
- crossterm
- tokio
- notify-rust
- serde + toml

---

## License

MIT
