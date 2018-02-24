#![allow(bad_style)] // Stops rust from complaining about non-camelCased structs (ATAG makes camelCase awkward)

use core::intrinsics::volatile_load;

// ATAG IDs
const ATAG_NONE:      u32 = 0x00000000;
const ATAG_CORE:      u32 = 0x54410001;
const ATAG_MEM:       u32 = 0x54410002;
const ATAG_VIDEOTEXT: u32 = 0x54410003;
const ATAG_RAMDISK:   u32 = 0x54410004;
const ATAG_INITRD2:   u32 = 0x54410005;
const ATAG_SERIAL:    u32 = 0x54410006;
const ATAG_REVISION:  u32 = 0x54410007;
const ATAG_VIDEOLFB:  u32 = 0x54410008;
const ATAG_CMDLINE:   u32 = 0x54410009;

struct ATAG_Header {
    size: u32, // length of tag in words, including header
    id: u32,
}

struct ATAG_Core {
    flags: u32,
    pageSize: u32,
    rootDev: u32,
}

pub struct ATAG_Mem {
    pub size: u32,
    pub start: u32,
}

struct ATAG_Videotext {
    x: u8,
    y: u8,
    video_page: u16,
    video_mode: u8,
    video_cols: u8,
    video_ega_bx: u16,
    video_lines: u8,
    video_isvga: u8,
    video_points: u16,
}

struct ATAG_Ramdisk {
    flags: u32,
    size: u32,
    start: u32,
}

struct ATAG_Initrd2 {
    start: u32,
    size: u32,
}

struct ATAG_Serialnr {
    low: u32,
    high: u32,
}

struct ATAG_Revision {
    rev: u32,
}

struct ATAG_Videolfb {
    lfb_width: u16,
    lfb_height: u16,
    lfb_depth: u16,
    lfb_linelength: u16,
    lfb_base: u32,
    lfb_size: u32,
    red_size: u8,
    red_pos: u8,
    green_size: u8,
    green_pos: u8,
    blue_size: u8,
    blue_pos: u8,
    rsvd_size: u8,
    rsvd_pos: u8,
}

struct ATAG_Cmdline {
    cmdline: [u8; 1], //minimum size: array of u8, size 1
}

//addr will be the place to start searching
pub fn getMemTag (mut addr: u32) -> Option<ATAG_Mem> {
    loop {
        let atag = unsafe { volatile_load(addr as *const ATAG_Header) };
        if atag.id == ATAG_MEM {
            return unsafe { Some(volatile_load(addr as *const ATAG_Mem)) };
        }
        else if atag.id == ATAG_NONE {
            return None;
        }
        addr += atag.size;
    }
}
