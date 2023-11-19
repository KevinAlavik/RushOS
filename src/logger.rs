use crate::vga_buffer::{WRITER, Color, ColorCode};

pub enum LogLevel {
    OK,
    WARNING,
    ERROR,
    PANIC,
    DEBUG,
    CUSTOM { tag: &'static str, tag_color: Color },
}

pub fn log(level: LogLevel, message: &str) {
    match level {
        LogLevel::OK => log_with_color(Color::Green, "OK", message),
        LogLevel::WARNING => log_with_color(Color::Yellow, "WARNING", message),
        LogLevel::ERROR => log_with_color(Color::Red, "ERROR", message),
        LogLevel::PANIC => log_with_color(Color::Red, "PANIC", message),
        LogLevel::DEBUG => log_with_color(Color::Cyan, "DEBUG", message),
        LogLevel::CUSTOM { tag, tag_color } => log_with_color(tag_color, tag, message),
    }
}

pub fn slog(level: LogLevel, message: &str) {
    match level {
        LogLevel::OK => slog_with_color(Color::Green, "OK", message),
        LogLevel::WARNING => slog_with_color(Color::Yellow, "WARNING", message),
        LogLevel::ERROR => slog_with_color(Color::Red, "ERROR", message),
        LogLevel::PANIC => slog_with_color(Color::Red, "PANIC", message),
        LogLevel::DEBUG => slog_with_color(Color::Cyan, "DEBUG", message),
        LogLevel::CUSTOM { tag, tag_color } => slog_with_color(tag_color, tag, message),
    }
}

fn log_with_color(color: Color, tag: &str, message: &str) {
    WRITER.lock().set_color(Color::White, Color::Black);
    print!("[ ");
    WRITER.lock().set_color(color, Color::Black);
    print!("{} ", tag);
    WRITER.lock().set_color(Color::White, Color::Black);
    print!("] {}", message);
    WRITER.lock().set_color(Color::White, Color::Black);
    println!();
}

fn slog_with_color(color: Color, tag: &str, message: &str) {
    WRITER.lock().set_color(Color::White, Color::Black);
    print!("[ ");
    WRITER.lock().set_color(color, Color::Black);
    print!("{} ", tag);
    WRITER.lock().set_color(Color::White, Color::Black);
    print!("] {}", message);
    WRITER.lock().set_color(Color::White, Color::Black);
}