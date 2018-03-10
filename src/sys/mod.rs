pub mod uart;

use core::intrinsics::volatile_load;
use core::intrinsics::volatile_store;

pub fn mmio_write(reg: u32, val: u32) {
    unsafe { volatile_store(reg as *mut u32, val) }
}

pub fn mmio_read(reg: u32) -> u32 {
    unsafe { volatile_load(reg as *const u32) }
}

// Issues the NOP instruction for the number of cycles specified
pub fn delay(mut cycles: u32) {
    while cycles > 0 {
        unsafe { asm!("NOP" :::: "volatile" ); }
        cycles -= 1;
    }
}
