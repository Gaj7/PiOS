pub mod first_fit;
// pub mod buddy;

use core::intrinsics::size_of;
use core::ops::Deref;
use core::ops::DerefMut;
use mem::HEAP_ALLOC;
use uart; //debugging

pub struct Box<T>{ //<T: ?Sized> ?
    elem: *mut T,
}

impl<T> Box<T> {
    pub fn new(x: T) -> Box<T> {
        unsafe {
            //let addr: *mut T = HEAP_ALLOC.alloc( size_of::<T>() as u32).unwrap() as *mut T;
            let addr = match HEAP_ALLOC.alloc( size_of::<T>() as u32) { // allocate heap memory
                Some(a) => a,
                None => {
                    uart::write_str("Couldn't alloc!!!!!\n");
                    0
                },
            } as *mut T;
            *addr = x;                                                 // copy content to heap addr
            Box { elem: addr }
        }
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

impl<T> DerefMut for Box<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut (*self.elem) }
    }
}
