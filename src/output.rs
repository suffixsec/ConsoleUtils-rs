use crate::color::{RGB, set_text_color, reset_text_color};

pub fn rgb(prompt: &str, color: RGB) {
    set_text_color(color);
    println!("{}", prompt);
    reset_text_color();
}

pub fn gradient(text: &str, start_color: RGB, end_color: RGB) {
    let step = 1.0 / (text.len() - 1) as f64;

    for (i, ch) in text.chars().enumerate() {
        let current_r = (start_color.r as f64 * (1.0 - step * i as f64) + end_color.r as f64 * (step * i as f64)) as u8;
        let current_g = (start_color.g as f64 * (1.0 - step * i as f64) + end_color.g as f64 * (step * i as f64)) as u8;
        let current_b = (start_color.b as f64 * (1.0 - step * i as f64) + end_color.b as f64 * (step * i as f64)) as u8;

        let current_color = RGB::new(current_r, current_g, current_b);
        rgb(format!("{}", ch), current_color);
    }

    reset_text_color();
}