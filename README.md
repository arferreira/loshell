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
| `[ / ]` | Switch station |
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

All stations powered by [SomaFM](https://somafm.com) - listener-supported, commercial-free radio.

- Groove Salad - ambient/chill
- Drone Zone - atmospheric space music
- Lush - sensual vocals with chillout
- Deep Space One - deep ambient/space
- Vaporwaves - vaporwave aesthetic

## The Story

This project was built through vibe coding sessions: Opus 4.5, neovim, and way too much coffee. The idea was simple: I wanted a focused environment that lives in the terminal, where I already spend most of my time.

No browser tabs. No distractions. Just music, a timer, and the work.

## Requirements

- macOS or Linux
- Audio output device

## License

MIT
