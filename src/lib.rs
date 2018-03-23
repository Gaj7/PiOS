#![no_std]
#![feature(core_intrinsics, lang_items)] //used here
// #![feature(use_extern_macros)]
#![feature(asm)]                         //used in uart
#![feature(ptr_internals)]               //used in mem/alloc
#![allow(dead_code)] //complaining about unused code is really annoying in early development


// #[macro_use]
pub mod sys;
pub mod mem;
pub mod atag;
pub mod test;
//pub mod video;
//pub mod process;

use sys::uart;

#[no_mangle]
pub extern fn kernel_main(_r0: u32, _r1: u32, atags_addr: u32) -> ! {
    // UART init
    uart::init();
    uart::write_str("PiOS booted!\n");

    uart::write_str("atags start addr: ");
    uart::write_hex(atags_addr);
    uart::write_str("\n");

    // Memory init
    let atags = atag::parse_atags(atags_addr);
    let mem_tag = match atags.mem {
        Some(tag) => {
            uart::write_str("Mem tag found.\n");
            tag
        },
        None => {
            uart::write_str("No mem tag found.\n");
            // atag::AtagMem {
            //     size: 1024 * 1024 * 128, //128 MB
            //     start: 0,
            // }
            atag::AtagMem {
                size: 1024, //1 KB
                start: 128,
            }
        },
    };
    uart::write_str("Mem size: ");
    uart::write_u32(mem_tag.size);
    uart::write_str("\n\n");
    mem::init(mem_tag);

    // Tests
    // test::test_ff(); // This test really shouldn't be run while we are actually initiallizing our heap, the test would overlap with the real thing
    // test::test_box();
    test::test_list();

    // Recieve/transmit loop
    loop {
        uart::write_c(uart::get_c())
    }
}


// These functions below provide definitions for symbols libcore
// expects which are not present on our bare metal target.
// Some of these should never be called, since we will abort on panic rather than unwind.

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(_: core::fmt::Arguments, _: &'static str, _: u32, _: u32) -> ! {
    loop {}
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn rust_eh_personality() {}

#[lang = "eh_unwind_resume"]
#[no_mangle]
pub extern fn rust_eh_unwind_resume() {}

#[no_mangle]
pub extern fn __aeabi_unwind_cpp_pr0() {}
