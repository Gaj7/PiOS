const HEADER_SIZE: u32 = 8; //or 5? better to be safe with 8
struct BlockHeader {        //would it be simpler if size didn't include header size?
    empty: bool,
    size: u32,
}

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

    pub fn alloc(&self, size: u32) -> Option<u32> {
        let mut curr_addr = self.begin;
        while curr_addr < self.end {
            unsafe {
                let curr_block: &mut BlockHeader = &mut *(curr_addr as *mut BlockHeader);
                if curr_block.empty && (curr_block.size - HEADER_SIZE) >= size {
                    curr_block.empty = false;
                    if curr_block.size - size > HEADER_SIZE {
                        *((curr_addr + size + HEADER_SIZE) as *mut BlockHeader) = BlockHeader {
                            empty: true,
                            size: (curr_block.size - size) + HEADER_SIZE,
                        };
                        curr_block.size = size + HEADER_SIZE;
                    }
                    return Some(curr_addr + HEADER_SIZE);
                }
                else {
                    curr_addr += curr_block.size;
                }
            }
        }
        None
    }

    //pub fn free(&self, )
}
