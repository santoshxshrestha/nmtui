# nmtui

A Terminal UI (TUI) for NetworkManager, built in Rust as a simple wrapper around `nmcli`.

> **Learning project:** This app was created mainly for learning Rust, exploring TUI design, and experimenting with how terminal applications work. It’s not meant to be super polished or perfect-just a fun side project!

## What’s this?

It’s basically a text-based network manager for your Linux system (using NetworkManager), controlled from your terminal-like `nmtui`, but home-brewed. You can:

- See available wifi networks
- Connect/disconnect networks
- Manage saved connections
- See current status-all in your terminal

## How do I use it?

### Build

You’ll need Rust (https://rustup.rs), then run:

```
cargo build --release
```

### Run

```
cargo run
```

...or run the compiled binary from `target/release/nmtui`.

## Notes

- Only tested on Linux (with NetworkManager installed)
- Some features might be a bit rough-pull requests & feedback are welcome!

## License

MIT (see LICENSE)

---
