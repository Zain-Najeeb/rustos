#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(aura_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};

use aura_os::println;
use core::panic::PanicInfo;

pub trait Testable {
    fn run(&self);
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{info}");
    aura_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    aura_os::test_panic_handler(info)
}

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use aura_os::memory;
    use x86_64::VirtAddr;

    println!("hello, world");
    aura_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut _mapper = unsafe { memory::init(phys_mem_offset) };
    let mut _frame_allocator =
        unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };

    #[cfg(test)]
    test_main();

    println!("it did not crash!");

    aura_os::hlt_loop();
}
