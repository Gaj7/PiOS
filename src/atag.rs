//#![allow(bad_style)] // Stops rust from complaining about non-camelCased structs (ATAG makes camelCase awkward)

// Note: the bootloader sets up the atags. Won't work within QEMU

use core::intrinsics::volatile_load;
//use uart; //for debug

// ATAG IDs
const NONE_ID:      u32 = 0x00000000;
const CORE_ID:      u32 = 0x54410001;
const MEM_ID:       u32 = 0x54410002;
const VIDEOTEXT_ID: u32 = 0x54410003;
const RAMDISK_ID:   u32 = 0x54410004;
const INITRD2_ID:   u32 = 0x54410005;
const SERIAL_ID:    u32 = 0x54410006;
const REVISION_ID:  u32 = 0x54410007;
const VIDEOLFB_ID:  u32 = 0x54410008;
const CMDLINE_ID:   u32 = 0x54410009;

struct AtagHeader {
    size: u32, // length of tag in words, including header
    id: u32,
}

pub struct AtagCore {
    pub flags: u32,
    pub page_size: u32,
    pub root_dev: u32,
}

pub struct AtagMem {
    pub size: u32,
    pub start: u32,
}

pub struct AtagVideotext {
    pub x: u8,
    pub y: u8,
    pub video_page: u16,
    pub video_mode: u8,
    pub video_cols: u8,
    pub video_ega_bx: u16,
    pub video_lines: u8,
    pub video_isvga: u8,
    pub video_points: u16,
}

pub struct AtagRamdisk {
    pub flags: u32,
    pub size: u32,
    pub start: u32,
}

pub struct AtagInitrd2 {
    pub start: u32,
    pub size: u32,
}

pub struct AtagSerial {
    pub low: u32,
    pub high: u32,
}

pub struct AtagRevision {
    pub rev: u32,
}

pub struct AtagVideolfb {
    pub lfb_width: u16,
    pub lfb_height: u16,
    pub lfb_depth: u16,
    pub lfb_linelength: u16,
    pub lfb_base: u32,
    pub lfb_size: u32,
    pub red_size: u8,
    pub red_pos: u8,
    pub green_size: u8,
    pub green_pos: u8,
    pub blue_size: u8,
    pub blue_pos: u8,
    pub rsvd_size: u8,
    pub rsvd_pos: u8,
}

pub struct AtagCmdline {
    pub cmdline: [u8; 1], //minimum size: array of u8, size 1
}

pub struct Atags {
    pub core: Option<AtagCore>,
    pub mem: Option<AtagMem>,
    pub videotext: Option<AtagVideotext>,
    pub ramdisk: Option<AtagRamdisk>,
    pub initrd2: Option<AtagInitrd2>,
    pub serial: Option<AtagSerial>,
    pub revision: Option<AtagRevision>,
    pub videolfb: Option<AtagVideolfb>,
    pub cmdline: Option<AtagCmdline>,
}
impl Atags {
    fn new() -> Atags {
        Atags {
            core: None,
            mem: None,
            videotext: None,
            ramdisk: None,
            initrd2: None,
            serial: None,
            revision: None,
            videolfb: None,
            cmdline: None,
        }
    }
}

// Assumes no more than one of each type of atag
// addr represents starting address of atags
pub fn parse_atags (mut addr: u32) -> Atags {
    let mut atags = Atags::new();
    loop {
        let atag = unsafe { volatile_load(addr as *const AtagHeader) };
        if atag.id == CORE_ID {
            atags.core = unsafe { Some(volatile_load((addr+2) as *const AtagCore)) }; //2 = sizeof the header
        } else if atag.id == MEM_ID {
            atags.mem = unsafe { Some(volatile_load((addr+2) as *const AtagMem)) };
        } else if atag.id == VIDEOTEXT_ID {
            atags.videotext = unsafe { Some(volatile_load((addr+2) as *const AtagVideotext)) };
        } else if atag.id == RAMDISK_ID {
            atags.ramdisk = unsafe { Some(volatile_load((addr+2) as *const AtagRamdisk)) };
        } else if atag.id == INITRD2_ID {
            atags.initrd2 = unsafe { Some(volatile_load((addr+2) as *const AtagInitrd2)) };
        } else if atag.id == SERIAL_ID {
            atags.serial = unsafe { Some(volatile_load((addr+2) as *const AtagSerial)) };
        } else if atag.id == REVISION_ID {
            atags.revision = unsafe { Some(volatile_load((addr+2) as *const AtagRevision)) };
        } else if atag.id == VIDEOLFB_ID {
            atags.videolfb = unsafe { Some(volatile_load((addr+2) as *const AtagVideolfb)) };
        } else if atag.id == CMDLINE_ID {
            atags.cmdline = unsafe { Some(volatile_load((addr+2) as *const AtagCmdline)) };
        } else if atag.id == NONE_ID {
            break;
        }
        addr += atag.size;
    }
    atags
}

// Deprecated: use parse_atags
pub fn get_mem_tag (mut addr: u32) -> Option<AtagMem> {
    loop {
        let atag = unsafe { volatile_load(addr as *const AtagHeader) };
        if atag.id == MEM_ID {
            return unsafe { Some(volatile_load((addr+2) as *const AtagMem)) }; //2 = sizeof the header
        }
        else if atag.id == NONE_ID {
            return None;
        }
        addr += atag.size;
    }
}
