#![no_std]
#![feature(core_intrinsics, lang_items)] //used here
#![feature(asm)]                         //used in uart
#![allow(dead_code)] //complaining about unused code is really annoying in early development

use core::intrinsics::abort;

pub mod uart;
pub mod atag;
pub mod mem;
pub mod video;
//pub mod process;

#[no_mangle]
pub extern fn kernel_main(_r0: u32, _r1: u32, atags_addr: u32) {
    uart::init();
    uart::write("PiOS booted!\n");

    uart::write("atags start addr: ");
    uart::write_hex(atags_addr);
    uart::write("\n");

    let atags = atag::parse_atags(atags_addr);
    let mem_size = match atags.mem {
        Some(tag) => {
            uart::write("Mem tag found.\n");
            tag.size
        },
        None => {
            uart::write("No mem tag found.\n");
            1024 * 1024 * 128
        },
    };
    uart::write("Mem size: ");
    uart::write_u32(mem_size);
    uart::write("\n\n");

    test_ff();

    loop {
        uart::write_c(uart::get_c())
    }
}

fn test_ff() {
    uart::write("Testing: First fit memory allocator:\nSetting range 0 to 256.\n");
    let ff = mem::first_fit::FirstFitAlloc::new(0,256);
    ff.debug_print();
    uart::write("Allocating 50 bytes:\n");
    ff.alloc(50);
    ff.debug_print();
    uart::write("Allocating 74 bytes:\n");
    let test_alloc = (ff.alloc(74)).unwrap();
    ff.debug_print();
    uart::write("Allocating 100 bytes:\n");
    ff.alloc(100);
    ff.debug_print();
    uart::write("Freeing our second allocation:\n");
    ff.free(test_alloc);
    ff.debug_print();
    uart::write("Allocating 40 bytes:\n");
    ff.alloc(40);
    ff.debug_print();
}

// These functions below provide definitions for symbols libcore
// expects which are not present on our bare metal target.

#[no_mangle]
pub extern fn __aeabi_unwind_cpp_pr0() {}

#[lang = "panic_fmt"]
pub extern fn panic_fmt(_: core::fmt::Arguments, _: &'static str, _: u32) -> ! {
    unsafe { abort() }
}

// There has got to be a better solution than this...
// This panic defn is expected when attempting plain ol' int addition
#[no_mangle]
pub extern fn _ZN4core9panicking5panic17h55432cad82b6074eE() -> ! {
    unsafe { abort() }
}
