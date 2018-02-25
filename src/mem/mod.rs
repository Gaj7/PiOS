
//use atag;

//pub mod alloc;

// pub fn init (mem_tag: atag::ATAG_Mem) {
//
// }

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
