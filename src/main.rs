#![no_std]
#![no_main]
#![feature(core_intrinsics)]
#![feature(lang_items)]

use core::panic::PanicInfo;
use x86_64::instructions::{hlt};
use crate::screen::*;
mod screen;

#[panic_handler]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop { hlt() }
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    welcome();

    loop { hlt(); }
}