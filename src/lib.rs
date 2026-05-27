//! # ask_input — простой ввод данных в Rust
//!
//! Библиотека делает ввод с клавиатуры таким же удобным, как в Python.
//! Одна функция на все случаи жизни.
//!
//! ## Быстрый старт
//! ```
//! use ask_input::input;
//!
//! let age: i32 = input().unwrap();
//! println!("Тебе {} лет", age);
//!
//! let name: String = input().unwrap();
//! println!("Привет, {}!", name);
//! ```

use std::io;
use std::str::FromStr;

/// Вводит значение с клавиатуры и парсит в нужный тип.
/// 
/// Тип определяется автоматически по типу переменной.
/// 
/// # Пример
/// ```
/// let age: i32 = ask_input::input().unwrap();
/// println!("Тебе {} лет", age);
/// 
/// let name: String = ask_input::input().unwrap();
/// println!("Привет, {}!", name);
/// 
/// let price: f64 = ask_input::input().unwrap();
/// println!("Цена: {} руб.", price);
/// ```
pub fn input<T: FromStr>() -> Result<T, Box<dyn std::error::Error>>
where
    <T as FromStr>::Err: std::error::Error + 'static,
{
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    Ok(buf.trim().parse::<T>()?)
}
