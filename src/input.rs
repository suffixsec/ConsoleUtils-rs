use crate::color::RGB;
use crate::output::{print_gradient, print_rgb};
use crate::console::flush_console;

pub fn input(prompt: &str) -> String {
    print!("{}", prompt);
    flush_console();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

pub fn input_rgb(prompt: &str, color: RGB) -> String {
    print_rgb(format!("{}", prompt).as_str(), color);
    flush_console();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

pub fn input_gradient(prompt: &str, start_color: RGB, end_color: RGB) -> String {
    print_gradient(format!("{}", prompt).as_str(), start_color, end_color);
    flush_console();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}