#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(aura_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use aura_os::println;
use core::panic::PanicInfo;

pub trait Testable {
    fn run(&self);
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{info}");
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    aura_os::test_panic_handler(info)
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("hello, world");

    #[cfg(test)]
    test_main();

    #[allow(clippy::empty_loop)]
    loop {}
}
