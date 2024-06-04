pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGB {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        RGB { r, g, b }
    }
}

pub fn set_text_color(color: RGB) {
    print!("\x1b[38;2;{};{};{}m", color.r, color.g, color.b);
}

pub fn reset_text_color() {
    print!("\x1b[0m");
}