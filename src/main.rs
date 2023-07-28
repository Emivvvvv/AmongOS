#![no_std]
#![no_main]
#![feature(core_intrinsics)]
#![feature(lang_items)]
#![feature(custom_test_frameworks)]
#![test_runner(amongos::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![allow(dead_code)]
#![allow(unused_imports)]

use core::panic::PanicInfo;
use amongos::{print, println};
use bootloader::{BootInfo, entry_point};
use x86_64::structures::paging::PageTable;
use x86_64::VirtAddr;

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

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    amongos::init();
    //



    //
    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    amongos::hlt_loop()
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    amongos::test_panic_handler(info)
}

