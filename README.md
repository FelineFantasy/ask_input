# ask_input ⌨️

[![Rust Version](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![docs.rs](https://img.shields.io/docsrs/ask_input)](https://docs.rs/ask_input)

A micro-library for keyboard input in Rust. No extra code — just input and get values!

## 📝 Description
`ask_input` is a tiny wrapper over Rust's standard I/O. One function. All types. Rust figures out the type automatically.

### Features:
- 🎯 One function for everything
- 🧠 Smart type detection
- ⚡ Zero dependencies

## ⚙️ Installation

Добавьте в ваш `Cargo.toml`:

```toml
[dependencies]
ask_input = "0.2.0"
```

## 🧪 Examples

Basic usage with proper error handling:

```rust
use ask_input::input;

fn main() {
    let age: i32 = input().expect("Failed to read age");
    let price: f64 = input().expect("Failed to read price");
    let name: String = input().expect("Failed to read name");
    
    println!("Age: {}, Price: {}, Name: {}", age, price, name);
}
```

## 📦 Functions
- `input::<T>()` — Input any type (i32, f64, String, etc.)

## 📋 Supported Types

| Type | Example | Notes |
|------|---------|-------|
| `i32` | `42` | Leading/trailing whitespace trimmed |
| `f64` | `3.14` | Leading/trailing whitespace trimmed |
| `String` | `Hello` | Whitespace preserved (only newline removed) |
| `bool` | `true` | Case-sensitive, whitespace trimmed |
| `i64`, `u32`... | Any numeric | Whitespace trimmed |

## ⚠️ Breaking Changes (v0.1.0 → v0.2.0)
- `int_input()` → `input::<i32>()`
- `float_input()` → `input::<f64>()`
- `str_input()` → `input::<String>()`
- Now returns `Result` instead of panicking

## 👤 Author
- **FelineFantasy**
- **License**: MIT