# loshell

A room for your mind, inside your terminal.

Loshell is a terminal-native ambient environment for deep focus. It blends lofi radio streams, a pomodoro timer, and a minimal interface into a calm, distraction-free experience.

Enter the room. Set the cycle. Drift into focus.

![demo](demo.gif)

## Install

```bash
cargo install loshell
```

Or via Homebrew:

```bash
brew tap arferreira/tap
brew install loshell
```

## Usage

```bash
loshell
```

### Keybindings

| Key | Action |
|-----|--------|
| `q` | Quit |
| `s` | Play/stop radio |
| `←/→` | Switch station |
| `p` | Toggle pomodoro |
| `space` | Start/pause timer |
| `r` | Reset timer |
| `+` | Add 5 minutes |
| `t` | Toggle todo list |
| `n` | New task |
| `j/k` | Navigate tasks |
| `x` | Mark task done |
| `d` | Delete task |
| `Enter` | Track task with pomodoro |

### Stations

- Chillout
- Lounge
- Relax FM
- Smooth Jazz
- Ambient

## The Story

This project was built through vibe coding sessions: Opus 4.5, neovim, and way too much coffee. The idea was simple: I wanted a focused environment that lives in the terminal, where I already spend most of my time.

No browser tabs. No distractions. Just music, a timer, and the work.

## Requirements

- macOS or Linux
- Audio output device

## License

MIT
