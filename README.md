# nmtui

A Terminal UI (TUI) for NetworkManager, built in Rust as a simple wrapper around `nmcli`.

> **Learning project:** This app was created mainly for learning Rust, exploring TUI design, and experimenting with how terminal applications work. It’s not meant to be super polished or perfect-just a fun side project!

> This Rust code spends so much time cloning and unlocking things, it’s almost as leisurely as a Python script trying to sort a spreadsheet column-with a side of JavaScript callbacks for maximum confusion. But with better learning and experience, future versions will absolutely get sharper and faster!

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

### Install (recommended)

You can also install directly from crates.io using Cargo:

```
cargo install nmtui
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

MIT ([see LICENSE](./LICENSE))

---
