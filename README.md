# ask_input ⌨️

[![Rust Version](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

A micro-library for keyboard input in Rust. No extra code — just input and get values!

## 📝 Description

`ask_input` is a tiny wrapper over Rust's standard I/O. Three functions. No magic. Just input and receive.

### Features:
- 🎯 **Minimalism** — Only 3 functions
- ⚡ **Fast** — Parses and returns immediately
- 📦 **Lightweight** — Zero dependencies
- 🔌 **Simple** — Add to Cargo.toml and go

## ⚙️ Installation

### Cargo.toml
```toml
[dependencies]
ask_input = { git = "https://github.com/FelineFantasy/ask_input.git" }
```

### Or clone
```bash
git clone https://github.com/FelineFantasy/ask_input.git
```

## 🧪 Examples
```rust
use ask_input;

fn main() {
    let age = ask_input::int_input();
    println!("You are {} years old", age);

    let price = ask_input::float_input();
    println!("Price: {} RUB", price);

    let name = ask_input::str_input();
    println!("Hello, {}!", name);
}
```

## 📦 Functions
- `int_input()` — Input an integer (`i32`)
- `float_input()` — Input a float (`f64`)
- `str_input()` — Input a string (whitespace trimmed)

## 📋 Requirements
- Rust 1.70+
- Willingness to type

## 👤 Author
- **FelineFantasy**
- **License**: MIT
