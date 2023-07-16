#![no_std]
#![no_main]
#![feature(core_intrinsics)]
#![feature(lang_items)]

use core::fmt::Write;
use core::fmt;
use core::panic::PanicInfo;
use x86_64::instructions::{hlt};

#[allow(unused)]
#[derive(Clone, Copy)]
#[repr(u8)]
enum Color {
    Black = 0x0, White = 0xF,
    Blue = 0x1, BrightBlue = 0x9,
    Green = 0x2, BrightGreen = 0xA,
    Cyan = 0x3, BrightCyan = 0xB,
    Red = 0x4, BrightRed = 0xC,
    Magenta = 0x5, BrightMagenta = 0xD,
    Brown = 0x6, Yellow = 0xE,
    Gray = 0x7, DarkGray = 0x8,
}

struct Cursor {
    position: isize,
    foreground: Color,
    background: Color,
}

impl Cursor {
    fn color(&self) -> u8 {
        let fg = self.foreground as u8;
        let bg = (self.background as u8) << 4;
        fg | bg
    }

    fn print(&mut self, text: &[u8]) {
        let color = self.color();

        let framebuffer = 0xb8000 as *mut u8;

        for &charecter in text {
            unsafe {
                framebuffer.offset(self.position).write_volatile(charecter);
                framebuffer.offset(self.position + 1).write_volatile(color);
            }
            self.position += 2;
        }
    }

    fn draw_amongos(&mut self) {
        self.background = Color::Red;
        self.draw_line(&[38, 39, 40], 9);
        self.draw_line(&[37, 38, 39, 40, 41], 10);
        self.draw_line(&[36, 37, 38], 11);
        self.draw_line(&[36, 37, 38], 12);
        self.draw_line(&[36, 37, 38, 39, 40, 41], 13);
        self.draw_line(&[36, 37, 38, 39, 40, 41], 14);
        self.draw_line(&[37, 38, 40, 41], 15);
        self.background = Color::BrightCyan;
        self.draw_line(&[39, 40, 41, 42], 11);
        self.draw_line(&[39, 40, 41, 42], 12);
    }

    fn draw_line(&mut self, x_list: &[isize], y: isize) {
        let color = self.color();
        let framebuffer = 0xb8000 as *mut u8;


        for x in x_list {
            self.position = (80 * y + x) * 2;
            unsafe {
                framebuffer.offset(self.position).write_volatile(b' ');
                framebuffer.offset(self.position + 1).write_volatile(color);
            }
        }
    }
}

impl Write for Cursor {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.print(s.as_bytes());
        Ok(())
    }
}

#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    let mut cursor = Cursor {
        position: 0,
        foreground: Color::White,
        background: Color::Red,
    };
    for _ in 0..(80*25) {
        cursor.print(b" ")
    }

    cursor.position = 0;
    write!(cursor, "{}", _info).unwrap();

    loop {
        hlt()
    }
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let text = b"Among OS";
    let creator = b"Created by Emirhan Tala";

    let mut cursor = Cursor {
        position: 0,
        foreground: Color::Red,
        background: Color::Black,
    };

    for _ in 0..(80*25) {
        cursor.print(b" ")
    }

    cursor.position = 36 * 2;
    cursor.print(text);

    cursor.position = (80 * 24 +28) * 2;
    cursor.print(creator);

    cursor.position = 0;
    cursor.draw_amongos();

    loop {
        hlt();
    }
}