#![no_std]
#![feature(core_intrinsics, lang_items)] //used here
#![feature(asm)]                         //used in uart
// #![feature(libc)]
#![allow(dead_code)] //complaining about unused code is really annoying in early development

use core::intrinsics::abort;

mod uart;
mod atag;
//mod process;

#[no_mangle]
pub extern fn kernel_main(_r0: u32, _r1: u32, atags_addr: u32) {
    uart::init();
    uart::write("piOS booted!\n");

    uart::write("atags start addr: ");
    uart::write_hex(atags_addr as u32);
    uart::write("\n");

    let mem_tag = atag::getMemTag(atags_addr);
    match mem_tag {
        Option::Some(tag) => uart::write("Mem tag found.\n"),
        Option::None      => uart::write("No mem tag found.\n"),
    }

    loop {
        uart::writec(uart::getc())
    }
}

// These functions below provide definitions for symbols libcore
// expects which are not present on our bare metal target.

#[no_mangle]
pub extern fn __aeabi_unwind_cpp_pr0() {}

#[no_mangle]
pub extern fn __aeabi_memcpy (dest: *mut u8, src: *const u8, n: usize) -> *mut u8{
    let mut i = 0;
    while i < n {
        unsafe { *dest.offset(i as isize) = *src.offset(i as isize); }
        i += 1;
    }
    dest
}

#[lang = "panic_fmt"]
pub extern fn panic_fmt(_: core::fmt::Arguments, _: &'static str, _: u32) -> ! {
    unsafe { abort() }
}

// There has got to be a better solution than this...
#[no_mangle]
pub extern fn _ZN4core9panicking5panic17h55432cad82b6074eE() -> ! {
    unsafe { abort() }
}
