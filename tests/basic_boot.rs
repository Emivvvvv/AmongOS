#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(amongos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use amongos::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    amongos::hlt_loop()
}

fn test_runner(tests: &[&dyn Fn()]) {
    unimplemented!();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    amongos::test_panic_handler(info)
}

#[test_case]
fn test_println() {
    println!("test_println output");
}