use crate::color::RGB;
use crate::output::{print_gradient, print_rgb};

pub fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

pub fn input_rgb(prompt: &str, color: RGB) -> String {
    print_rgb(format!("{}", prompt), color);
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

pub fn input_gradient(prompt: &str, start_color: RGB, end_color: RGB) -> String {
    print_rgb(format!("{}", prompt), start_color, end_color);
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}