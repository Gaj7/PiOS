pub mod first_fit;
// pub mod buddy;

use core::intrinsics::size_of;
use mem::HEAP_ALLOC;

pub struct Box<T>{ //<T: ?Sized> ?
    elem: *mut T,
}

impl<T> Box<T> {
    pub fn new(x: T) -> Box<T> {
        unsafe {
            let addr: *mut T = HEAP_ALLOC.alloc( size_of::<T>() as u32).unwrap() as *mut T; // allocate heap memory
            *addr = x;                                                                      // copy content to heap addr
            Box { elem: addr }
        }
    }
    pub fn deref(&self) -> &T {
        unsafe { &*self.elem }
    }
    // Should manual deletion be allowed, or just let Drop take care of it?
    pub fn del(&self) {
        unsafe {
            HEAP_ALLOC.free(self.elem as u32);
        }
    }
}

// This overload of the drop function calls our delete function when a Box drops out of scope
impl<T> Drop for Box<T> {
    fn drop(&mut self) {
        self.del();
    }
}
