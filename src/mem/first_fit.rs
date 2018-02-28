pub struct FirstFitAlloc {
    begin: u32,
    end: u32,
}

impl FirstFitAlloc {
    pub fn new(begin: u32, end: u32) -> FirstFitAlloc {
        unsafe {
            *(begin as *mut BlockHeader) = BlockHeader {
                empty: true,
                size: (end - begin),
            };
        }
        FirstFitAlloc {
            begin: begin,
            end: end,
        }
    }
    pub fn alloc(size: u32) -> Option<u32> {
        None
    }
}

struct BlockHeader {
    empty: bool,
    size: u32,
}
