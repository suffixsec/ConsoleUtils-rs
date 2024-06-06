use std::io::{self, Write};

pub fn flush_console() {
    io::stdout().flush().unwrap();
}

pub fn clear_line() {
    print!("\r\x1b[2K");
    flush_console();
}

pub fn clear_lines(num: i32) {
    for _ in 0..num {
        print!("\r\x1b[2K");
        flush_console();
    }
}