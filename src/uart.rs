//#![feature(core_intrinsics)] //not needed because included in parent lib.rs
//#![feature(asm)] // ^ same

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

#[cfg(target_arch = "arm")]
fn delay(mut cycles: u32) {
    // unsafe { asm!( "__delay_%=: subs %[cycles], %[cycles], #1; bne __delay_%=\n"
    //              : "=r"(cycles)
    //              : [cycles]"0"(cycles)
    //              : "cc"
    //              : "volatile");
    //        }
    unsafe { asm!( "__delay_: subs $0, $0, #1; bne __delay_\n"
                 : "=r"(cycles)
                 : "0"(cycles)
                 : "cc"
                 : "volatile");
           }
}

fn transmit_fifo_full() -> bool {
    mmio_read(UART_FR) & (1 << 5) > 0
}

fn receive_fifo_empty() -> bool {
    mmio_read(UART_FR) & (1 << 4) > 0
}

// Will need to implement this to get it to run real raspi
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

pub fn writec(c: u8) {
    while transmit_fifo_full() {}
    mmio_write(UART_DR, c as u32);
}

pub fn getc() -> u8 {
    while receive_fifo_empty() {}
    mmio_read(UART_DR) as u8
}

pub fn write(msg: &str) {
    for c in msg.chars() {
        writec(c as u8)
    }
}
