use std::io;

fn main() {
    println!("🌡 Температурний конвертер: Цельсій ↔ Фаренгейт");

    println!("Введіть температуру у Цельсіях:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Помилка вводу");

    let celsius: f64 = input.trim().parse().expect("Це має бути число");

    let fahrenheit = celsius * 9.0 / 5.0 + 32.0;
    println!("{:.2}°C = {:.2}°F", celsius, fahrenheit);
}
