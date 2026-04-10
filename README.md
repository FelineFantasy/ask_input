# simple_input ⌨️

[![Rust Version](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

A micro-library for simple keyboard input in Rust. No extra code — just input and get values!

## 📋 Table of Contents
- [Description](#description)
- [How to Use](#how-to-use)
- [Installation](#installation)
- [Examples](#examples)
- [Functions](#functions)
- [Requirements](#requirements)
- [Author](#author)

## 📝 Description

**simple_input** is a tiny wrapper library over Rust's standard I/O. It allows you to easily get numbers and strings from users without writing verbose boilerplate code every time.

### Features:
- 🎯 **Minimalism** — Only 3 functions for everything
- 🔄 **Ready to use** — Parses and returns data immediately
- 📦 **Lightweight** — Zero extra dependencies
- 🔌 **Simple integration** — Add to `Cargo.toml` and go

## 🎮 How to Use

1. Add the library to your project
2. Call the desired input function
3. Use the received value

## ⚙️ Installation

### Option 1: Add to Cargo.toml manually

```toml
[dependencies]
simple_input = { git = "https://github.com/FelineFantasy/ask_input.git" }
```

### Option 2: Clone the repository

```bash
git clone https://github.com/FelineFantasy/ask_input.git
```

## 🧪 Examples

```rust
use simple_input;

let number = simple_input::get_number::<i32>();
let text = simple_input::get_string();
```

## 📦 Functions

- `get_number<T>()` — Gets a number of type T from input
- `get_string()` — Gets a string from input
- `get_number_range<T>(min, max)` — Gets a number within a range

## 📋 Requirements

- Rust 1.70+
- Cargo

## 👤 Author

**FelineFantasy**

License: MIT
