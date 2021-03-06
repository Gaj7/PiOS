use uart;
use mem::alloc::first_fit::FirstFitAlloc;
use mem::alloc::Box;
use mem::data_structs::list::List;

pub fn test_ff() {
    uart::write_str("Testing: First fit memory allocator:\nSetting range 0 to 256.\n");
    let ff = FirstFitAlloc::new(0,256);
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

pub fn test_box() {
    uart::write_str("Testing: heap memory allocation.\nAllocating the number 5 on the heap.\n");
    let heap_num = Box::new(5 as u32);
    uart::write_str("Reading number from our box struct: ");
    uart::write_u32(*heap_num);
    uart::write_str(".\nDeleting...\n");
    heap_num.del();
    uart::write_str("Trying to access it (might still work, if it hasn't been allocated over): ");
    uart::write_u32(*heap_num);
    uart::write_str("\nAllocating the letter 'w' on the heap.\n");
    let heap_num2 = Box::new('w' as u8);
    uart::write_str("Reading letter from our box struct: ");
    uart::write_c(*heap_num2);
    uart::write_str("\nNow trying to access our number again (likely written over now): ");
    uart::write_u32(*heap_num);
    uart::write_str(".\nDeleting our letter...\n");
    heap_num2.del();
}

pub fn test_list() {
    uart::write_str("Pushing 1..5 to the stack.\n");
    let mut stack = List::<u32>::new();
    let mut i = 1 as u32;
    while i <= 5 {
        uart::write_str("Pushing ");
        uart::write_u32(i);
        uart::write_str("\n");
        stack.push(i);
        i += 1;
    }
    uart::write_str("Popping each element off the stack:\n");
    i = 0;
    while i < 5 {
        uart::write_u32(stack.pop().unwrap());
        uart::write_str(", ");
        i += 1;
    }
    uart::write_c('\n' as u8);
}
