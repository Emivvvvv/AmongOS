#![no_std]
#![no_main]
#![feature(core_intrinsics)]
#![feature(lang_items)]
#![feature(custom_test_frameworks)]
#![test_runner(amongos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use amongos::{println};

#[panic_handler]
#[no_mangle]
#[cfg(not(test))]
pub fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    amongos::hlt_loop()
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    amongos::init();

    #[cfg(test)]
    test_main();

    amongos::hlt_loop()
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    amongos::test_panic_handler(info)
}

