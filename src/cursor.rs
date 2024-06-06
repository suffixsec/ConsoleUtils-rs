use crate::console::flush_console;

pub struct Visibility;

impl Visibility {
    pub fn show_cursor() {
        print!("\x1B[?25h");
        flush_console();
    }

    pub fn hide_cursor() {
        print!("\x1B[?25l");
        flush_console();
    }
}

pub struct Move {
    pub fn Up(num: i32) {
        if num > 0 {
            print!("\x1b[{}A", num);
            flush_console();
        }
    }

    pub fn Down(num: i32) {
        if num > 0 {
            print!("\x1b[{}B", num);
            flush_console();
        }
    }

    pub fn Left(num: i32) {
        if num > 0 {
            print!("\x1b[{}D", num);
            flush_console();
        }
    }

    pub fn Right(num: i32) {
        if num > 0 {
            print!("\x1b[{}C", num);
            flush_console();
        }
    }
}