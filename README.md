![Rust](https://github.com/DankDumpster/mouse-rs/workflows/Rust/badge.svg?style=flat-square) [![Docs](https://docs.rs/mouse-rs/badge.svg?style=flat-square)](https://docs.rs/mouse-rs) [![Crates.io](https://img.shields.io/crates/v/mouse-rs)](https://crates.io/crates/mouse-rs)
# Mouse-rs

Mouse-rs is a rust library for controlling your mouse from a rust program, without having to go into your kernel yourself.

This project was loosely based on the python [mouse](https://github.com/boppreh/mouse/) library.
Currently it supports macos, windows and linux! If you need any other OS added please open an [issue](https://github.com/DankDumpster/mouse-rs/issues/new)

## Installation
Add mouse-rs to your cargo.toml

```toml
[dependencies]
mouse-rs = "0.4"
```

## Example
This is a simple example that moves your mouse to a position on screen and presses the left button.

```rust
use mouse_rs::{types::keys::Keys, Mouse};

fn move_and_press() {
    let mouse = Mouse::new();
    mouse.move_to(500, 500).expect("Unable to move mouse");
    mouse.press(&Keys::RIGHT).expect("Unable to press button");
    mouse.release(&Keys::RIGHT).expect("Unable to release button");
}
```


## Usage
For more information please visit the [docs](https://docs.rs/mouse-rs/*/mouse_rs/)

## Linux disclaimer
If you're running into problems building on linux you need to install libxdo-dev.

#### Debian-based
```bash
sudo apt-get install -y libxdo-dev
```

#### Arch
```bash
sudo pacman -Sy xdotool
```

#### Fedora
```bash
sudo dnf install libX11-devel libxdo-devel
```

#### Gentoo
```bash
sudo emerge xdotool
```
