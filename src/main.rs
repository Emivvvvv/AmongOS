#![no_std]
#![no_main]
#![feature(core_intrinsics)]
#![feature(lang_items)]
#![feature(custom_test_frameworks)]
#![test_runner(amongos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use x86_64::instructions::{hlt};
use amongos::{println};

#[panic_handler]
#[no_mangle]
#[cfg(not(test))]
pub fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop { hlt() }
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    amongos::init();

    #[cfg(test)]
    test_main();

    loop { hlt(); }
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    amongos::test_panic_handler(info)
}

