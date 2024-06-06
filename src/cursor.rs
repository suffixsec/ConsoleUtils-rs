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

pub struct Move;

impl Move {
    pub fn cursor_up(num: i32) {
        if num > 0 {
            print!("\x1b[{}A", num);
            flush_console();
        }
    }

    pub fn cursor_down(num: i32) {
        if num > 0 {
            print!("\x1b[{}B", num);
            flush_console();
        }
    }

    pub fn cursor_left(num: i32) {
        if num > 0 {
            print!("\x1b[{}D", num);
            flush_console();
        }
    }

    pub fn cursor_right(num: i32) {
        if num > 0 {
            print!("\x1b[{}C", num);
            flush_console();
        }
    }
}