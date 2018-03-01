//Note: due to u32 typing of size, this implementation does not support larger ranges than 1 << 32 = 4GB

use core::intrinsics::size_of;
use uart; //for debug print

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
    // Initializes allocator over range, with a single empty block over entire range
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

    // Combines neighboring free blocks of memory during traversal, returns address corresponding to first free block of memory large enough to fullfill the request.
    pub fn alloc(&self, size: u32) -> Option<u32> {
        let mut curr_addr = self.begin;
        while curr_addr < self.end {
            unsafe {
                let curr_block: &mut BlockHeader = &mut *(curr_addr as *mut BlockHeader);
                if curr_block.empty {
                    // if block is big enough, return a chunk of this one
                    if (curr_block.size - HEADER_SIZE) >= size {
                        curr_block.empty = false;
                        if curr_block.size - size > HEADER_SIZE {
                            *((curr_addr + size + HEADER_SIZE) as *mut BlockHeader) = BlockHeader {
                                empty: true,
                                size: (curr_block.size - size) - HEADER_SIZE,
                            };
                            curr_block.size = size + HEADER_SIZE;
                        }
                        return Some(curr_addr + HEADER_SIZE);
                    }
                    // else if next block is free, combine them
                    else {
                        let next_block: &mut BlockHeader = &mut *((curr_addr + curr_block.size) as *mut BlockHeader);
                        if next_block.empty {
                            curr_block.size += next_block.size;
                        }
                        else {
                            curr_addr += curr_block.size;
                        }
                    }
                }
                else {
                    curr_addr += curr_block.size;
                }
            }
        }
        // traversed entire memory without finding a valid candidate
        None
    }

    // Simply marks address as empty
    pub fn free(&self, addr: u32) {
        unsafe {
            let curr_block: &mut BlockHeader = &mut *((addr - HEADER_SIZE) as *mut BlockHeader);
            curr_block.empty = true;
        }
    }

    pub fn debug_print(&self) {
        let mut curr_addr = self.begin;
        let mut block_num = 0;
        while curr_addr < self.end {
            unsafe {
                let curr_block: &mut BlockHeader = &mut *(curr_addr as *mut BlockHeader);
                uart::write_str("Block ");
                uart::write_u32(block_num);
                uart::write_str(":\tAddress: ");
                uart::write_u32(curr_addr);
                uart::write_str(",\tSize: ");
                uart::write_u32(curr_block.size);
                uart::write_str( if curr_block.empty {",\tEmpty.\n"} else {",\tFull.\n"});

                curr_addr += curr_block.size;
                block_num += 1;
            }
        }
    }
}
