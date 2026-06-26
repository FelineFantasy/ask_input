//! # ask_input — Simple data input in Rust
//!
//! This library makes terminal input in Rust as easy and convenient as it is in Python.
//! One single generic function to handle all your input needs with automatic stdout flushing.
//!
//! ## Quick Start
//! ```
//! use ask_input::input;
//!
//! let age: i32 = input().unwrap();
//! println!("You are {} years old", age);
//!
//! let name: String = input().unwrap();
//! println!("Hello, {}!", name);
//! ```

use std::io::{self, Write};
use std::str::FromStr;

/// Reads a line from stdin, flushes stdout automatically, and parses it into the target type.
/// 
/// The target type is inferred automatically based on the variable type assignment.
/// 
/// # Examples
/// ```
/// let age: i32 = ask_input::input().unwrap();
/// println!("Age: {}", age);
/// 
/// let name: String = ask_input::input().unwrap();
/// println!("Hello, {}!", name);
/// 
/// let price: f64 = ask_input::input().unwrap();
/// println!("Price: \${}", price);
/// ```
pub fn input<T: FromStr>() -> Result<T, Box<dyn std::error::Error>>
where
    <T as FromStr>::Err: std::error::Error + 'static,
{
    let _ = io::stdout().flush();

    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    Ok(buf.trim().parse::<T>()?)
}
