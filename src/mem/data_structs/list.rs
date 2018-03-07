use core::mem::replace;
use core::mem::uninitialized;
use mem::alloc::Box;

struct ListNode<T> {
    elem: T,
    next: Option<Box<ListNode<T>>>,
}

pub struct List<T> {
    head: Option<Box<ListNode<T>>>,
}
impl<T> List<T> {
    pub fn new() -> List<T> {
        List { head: None }
    }
    // TODO: add printlns to pinpoint hang loc when pushing to nonempty list
    pub fn push(&mut self, e: T) {
        let node = Box::new(ListNode {
            elem: e,
            next: self.head.take(),
        });
        self.head = Some(node);
    }
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            let borrowed_node: &mut ListNode<T> = node.borrow_mut();
            self.head = borrowed_node.next.take();
            replace(&mut borrowed_node.elem, unsafe{ uninitialized()} )
        })
    }
}
