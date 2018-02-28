//Note: due to u32 typing of size, this implementation does not support larger ranges than 1 << 32 = 4GB

use core::intrinsics::size_of;

const HEADER_SIZE: u32 = unsafe { size_of::<BlockHeader>() } as u32;
struct BlockHeader {
    empty: bool,
    size: u32, //size of block in bytes, INCLUDING header size, which is 8;
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

    //TODO: combine neighboring free blocks during traversal
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

    pub fn free(&self, addr: u32) {
        unsafe {
            let curr_block: &mut BlockHeader = &mut *((addr - HEADER_SIZE) as *mut BlockHeader);
            curr_block.empty = true;
        }
    }
}
