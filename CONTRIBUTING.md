# Contributing to loshell

Thanks for wanting to help improve loshell.

## Getting Started

```bash
git clone https://github.com/arferreira/loshell.git
cd loshell
cargo run
```

## What We're Looking For

- New radio stations (must be direct HTTP streams, no YouTube)
- UI improvements (keeping it minimal)
- Bug fixes
- Performance improvements
- Platform support (Windows, etc.)

## Pull Requests

1. Fork the repo
2. Create a branch (`git checkout -b feature/your-thing`)
3. Make your changes
4. Run `cargo fmt` and `cargo clippy`
5. Commit with a clear message
6. Push and open a PR

Keep PRs focused. One feature or fix per PR.

## Code Style

- Run `cargo fmt` before committing
- No warnings from `cargo clippy`
- Keep it simple - this is a small focused tool

## Adding Stations

Stations live in `src/radio.rs`. To add one:

```rust
Station {
    name: "Station Name",
    url: "https://direct-stream-url.mp3",
},
```

Make sure the stream is reliable and publicly accessible.

## Questions?

Open an issue.
