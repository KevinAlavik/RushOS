#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[macro_use]
mod vga_buffer;

mod logger;

use crate::vga_buffer::{Color, BUFFER_HEIGHT, BUFFER_WIDTH};
use logger::{LogLevel, log, slog};

fn divide(a: i32, b: i32) -> Result<i32, &'static str> {
    Ok(a / b)
}

#[no_mangle]
pub extern "C" fn _start() {
    log(LogLevel::CUSTOM { tag: "Kernel Info", tag_color: Color::LightGreen }, "Version: 0.1");
    log(LogLevel::CUSTOM { tag: "Kernel Info", tag_color: Color::LightGreen }, "Not stable (Early developement)");
    log(LogLevel::DEBUG, "Attempting division by zero.");
    divide(1, 0);
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    slog(LogLevel::PANIC, "");
    print!("{}", info);
    loop {}
}
