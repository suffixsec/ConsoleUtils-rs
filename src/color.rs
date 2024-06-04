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

pub fn SetTextColor(color: RGB) {
    print!("\033[38;2;{};{};{}m", color.r, color.g, color.b);
}

pub fn ResetTextColor() {
    print!("\033[0m");
}