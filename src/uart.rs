//#![feature(core_intrinsics, asm)]
#![allow(dead_code)] // Don't complain about unused values

use core::intrinsics::volatile_load;
use core::intrinsics::volatile_store;

// raspi2 and raspi3 have peripheral base address 0x3F000000,
// but raspi1 has peripheral base address 0x20000000.
const PERIPH_BASE: u32 = 0x3F000000;
const GPIO_BASE:   u32 = PERIPH_BASE + 0x200000;
const UART_BASE:   u32 = PERIPH_BASE + 0x201000;

const GPPUD:       u32 = GPIO_BASE + 0x94;
const GPPUDCLK0:   u32 = GPIO_BASE + 0x98;

const UART_DR:     u32 = UART_BASE + 0x00;
const UART_RSRECR: u32 = UART_BASE + 0x04;
const UART_FR:     u32 = UART_BASE + 0x18;
const UART_ILPR:   u32 = UART_BASE + 0x20;
const UART_IBRD:   u32 = UART_BASE + 0x24;
const UART_FBRD:   u32 = UART_BASE + 0x28;
const UART_LCRH:   u32 = UART_BASE + 0x2c;
const UART_CR:     u32 = UART_BASE + 0x30;
const UART_IFLS:   u32 = UART_BASE + 0x34;
const UART_IMSC:   u32 = UART_BASE + 0x38;
const UART_RIS:    u32 = UART_BASE + 0x3c;
const UART_MIS:    u32 = UART_BASE + 0x40;
const UART_ICR:    u32 = UART_BASE + 0x44;
const UART_DMACR:  u32 = UART_BASE + 0x48;
const UART_ITCR:   u32 = UART_BASE + 0x80;
const UART_ITIP:   u32 = UART_BASE + 0x84;
const UART_ITOP:   u32 = UART_BASE + 0x88;
const UART_TDR:    u32 = UART_BASE + 0x8c;

fn mmio_write(reg: u32, val: u32) {
    unsafe { volatile_store(reg as *mut u32, val) }
}

fn mmio_read(reg: u32) -> u32 {
    unsafe { volatile_load(reg as *const u32) }
}

// Issues the NOP instruction for the number of cycles specified
fn delay(mut cycles: u32) {
    while cycles > 0 {
        unsafe { asm!("NOP" :::: "volatile" ); }
        cycles -= 1;
    }
}

fn transmit_fifo_full() -> bool {
    mmio_read(UART_FR) & (1 << 5) > 0
}

fn receive_fifo_empty() -> bool {
    mmio_read(UART_FR) & (1 << 4) > 0
}

pub fn init() {
    // Disable the UART until it is ready
    mmio_write(UART_CR, 0x0);

    // Disable pull up/down for pins 14 and 15
    mmio_write(GPPUD, 0x0);
    delay(150);
    mmio_write(GPPUDCLK0, (1 << 14) | (1 << 15));
    delay(150);
    mmio_write(GPPUDCLK0, 0x0);

    // Set baud rate 1/40
    mmio_write(UART_IBRD, 1);
    mmio_write(UART_FBRD, 40);

    // Enable FIFO (4) and set word length to 8 bits (5 and 6)
    mmio_write(UART_LCRH, (1 << 4) | (1 << 5) | (1 << 6));

    // Mask all interrupts (bits 0, 2, 3, and 11+ all unused)
    mmio_write(UART_IMSC, (1 << 1) | (1 << 4) | (1 << 5) | (1 << 6) | (1 << 7) | (1 << 8) | (1 << 9) | (1 << 10));

    // Enable UART (0), enable transnmit and receive (8 and 9)
    mmio_write(UART_CR, (1 << 0) | (1 <<8 ) | (1 << 9));
}

pub fn write_c(c: u8) {
    while transmit_fifo_full() {}
    mmio_write(UART_DR, c as u32);
}

pub fn get_c() -> u8 {
    while receive_fifo_empty() {}
    mmio_read(UART_DR) as u8
}

pub fn write(msg: &str) {
    for c in msg.chars() {
        write_c(c as u8)
    }
}

pub fn write_u32(num: u32) {
    if num == 0 {
        write_c('0' as u8);
    }
    else if let Some(digit) = next_digit(num) {
        write_c(digit);
    }
}
// Helper function for write_num. Recursive solution takes advantage of backtracking to correct
// the order digits write in. A straightforward print loop would print in reverse order
fn next_digit(num: u32) -> Option<u8>{
    if num == 0 {
        return None
    }
    else {
        if let Some(digit) = next_digit(num/10) {
            write_c(digit);
        }
        return Some(((num % 10) + 0x30) as u8);
    }
}

pub fn write_i32(num: i32) {
    if num < 0 {
        write_c('-' as u8);
        write_u32((num * -1) as u32);
    }
    else {
        write_u32(num as u32);
    }
}

pub fn write_hex(num: u32) {
    write("0x");
    let mut i = 0 ;
    while i < 8 {
        let hexit = ((num >> (28 - 4*i) ) & 0xF) as u8;
        if hexit < 0xA {
            write_c(hexit + 0x30);
        }
        else {
            write_c(hexit + 0x37);
        }
        i += 1;
    }
}
