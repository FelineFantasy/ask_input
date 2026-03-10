use std::io;

/// Вводит целое число с клавиатуры
/// 
/// # Пример
/// ```
/// let age = int_input();
/// println!("Тебе {} лет", age);
/// ```
pub fn int_input() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

/// Вводит дробное число с клавиатуры
/// 
/// # Пример
/// ```
/// let price = float_input();
/// println!("Цена: {} руб.", price);
/// ```
pub fn float_input() -> f64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

/// Вводит строку с клавиатуры
/// 
/// Убирает пробелы и перенос строки в начале и конце
/// 
/// # Пример
/// ```
/// let name = str_input();
/// println!("Привет, {}!", name);
/// ```
pub fn str_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}