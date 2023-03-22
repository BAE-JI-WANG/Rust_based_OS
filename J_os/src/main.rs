#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(J_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use J_os::println;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    J_os::init();

    use x86_64::registers::control::Cr3;

    let (level_4_page_table, _) = Cr3::read();
    println!("Level 4 page table at: {:?}", level_4_page_table.start_address());

    // fn stack_overflow() {
    //     stack_overflow(); // for each recursion, the return address is pushed
    // }

    // uncomment line below to trigger a stack overflow
    // stack_overflow();
    // let ptr = 0xdeadbeaf as *mut u32;
    // unsafe { *ptr = 42; }

    // let ptr = 0x2031b2 as *mut u32;

    // // read from a code page
    // unsafe { let x = *ptr; }
    // println!("read worked");

    // // write to a code page
    // unsafe { *ptr = 42; }
    // println!("write worked");

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    J_os::hlt_loop();
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
    J_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}