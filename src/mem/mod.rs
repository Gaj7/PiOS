pub mod alloc;
pub mod data_structs;

use atag;
use mem::alloc::first_fit::FirstFitAlloc;

// Global var for heap allocator
static mut HEAP_ALLOC: FirstFitAlloc = FirstFitAlloc {begin: 0, end: 0};

pub fn init (mem_tag: atag::AtagMem) {
    unsafe {
        HEAP_ALLOC = FirstFitAlloc::new(mem_tag.start, mem_tag.size - mem_tag.start);
    }
}

// These functions below provide definitions for symbols libcore
// expects which are not present on our bare metal target.

#[no_mangle]
pub extern fn __aeabi_memcpy (dest: *mut u8, src: *const u8, n: usize) -> *mut u8{
    let mut i = 0;
    while i < n {
        unsafe { *dest.offset(i as isize) = *src.offset(i as isize); }
        i += 1;
    }
    dest
}

#[no_mangle]
pub extern fn __aeabi_memcpy4 (dest: *mut u8, src: *const u8, n: usize) -> *mut u8{
    __aeabi_memcpy (dest, src, n/4)
}
