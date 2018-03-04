pub mod first_fit;
// pub mod buddy;

use core::intrinsics::size_of;
use mem::HEAP_ALLOC;

pub struct Box<T: ?Sized>{
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
    pub fn del(&self) {
        unsafe {
            HEAP_ALLOC.free(self.elem as u32);
        }
    }
}

// Box is a smart pointer that dynamically allocates memory
// pub struct Box<T: ?Sized>(core::ptr::Unique<T>);
//
// impl<T> Box<T> {
//     fn new(x: T) -> Box<T> {
//         let addr = //ff.alloc(size_of(x));
//         //copy over??
//         Box(core::ptr::Unique::new_unchecked(1 as *mut T))
//     }
// }

// Deref trait is in std lib, look into overloading operations manually?
// impl<T> Deref for Box<T> {
//     type Target = T;
//
//     fn deref(&self) -> &T {
//         &self.0
//     }
// }
