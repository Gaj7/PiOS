pub mod first_fit;
// pub mod buddy;

use core::intrinsics::size_of;
use core::ops::Deref;
// use core::borrow::BorrowMut;
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

    pub fn borrow_mut(&self) -> &mut T {
        unsafe { &mut (*self.elem) }
    }

    // not callable publically - box will free heap memory when it drops out of scope
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

impl<T> Deref for Box<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &(*self.elem) }
    }
}
