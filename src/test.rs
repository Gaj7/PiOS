use uart;
use mem;

pub fn test_ff() {
    uart::write_str("Testing: First fit memory allocator:\nSetting range 0 to 256.\n");
    let ff = mem::first_fit::FirstFitAlloc::new(0,256);
    ff.debug_print();
    uart::write_str("Allocating 50 bytes:\n");
    ff.alloc(50);
    ff.debug_print();
    uart::write_str("Allocating 74 bytes:\n");
    let test_alloc = (ff.alloc(74)).unwrap();
    ff.debug_print();
    uart::write_str("Allocating 100 bytes:\n");
    ff.alloc(100);
    ff.debug_print();
    uart::write_str("Freeing our second allocation:\n");
    ff.free(test_alloc);
    ff.debug_print();
    uart::write_str("Allocating 40 bytes:\n");
    ff.alloc(40);
    ff.debug_print();
}
