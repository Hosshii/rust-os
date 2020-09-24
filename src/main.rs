#![no_std] // dont link rust standard library
#![no_main] // disable rust-level entry point
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

// mod serial;
// mod vga_buffer;

// extern crate rlibc;
use core::panic::PanicInfo;
use rust_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // panic!("_info: &PanicInfo");
    println!("hello world{}", "!");

    rust_os::init();

    // fn stack_overflow() {
    //     stack_overflow();
    // }

    // stack_overflow();

    unsafe {
        *(0xdeadbeef as *mut u64) = 42;
    }

    x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();

    println!("it did not crash");
    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info)
}
