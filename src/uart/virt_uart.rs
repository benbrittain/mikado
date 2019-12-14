#![allow(unused)]

use core::{fmt, ptr};

/// NS16550A UART Constants

const NS16550A_UART: *mut u8 = 0x10000000 as *mut u8;
const UART0_CLOCK_FREQ: u32 = 1843200;
const UART0_BAUD_RATE: u32 = 115200;
const DIVISOR: u32 = UART0_CLOCK_FREQ / (16 * UART0_BAUD_RATE);

const UART_RBR: u8 = 0x00; // Receive Buffer Register
const UART_THR: u8 = 0x00; // Transmit Hold Register
const UART_IER: u8 = 0x01; // Interrupt Enable Register
const UART_DLL: u8 = 0x00; // Divisor LSB (LCR_DLAB)
const UART_DLM: u8 = 0x01; // Divisor MSB (LCR_DLAB)
const UART_FCR: u8 = 0x02; // FIFO Control Register
const UART_LCR: u8 = 0x03; // Line Control Register
const UART_MCR: u8 = 0x04; // Modem Control Register
const UART_LSR: u8 = 0x05; // Line Status Register
const UART_MSR: u8 = 0x06; // Modem Status Register
const UART_SCR: u8 = 0x07; // Scratch Register

const UART_LCR_DLAB: u8 = 0x80; // Divisor Latch Bit
const UART_LCR_8BIT: u8 = 0x03; // 8-bit
const UART_LCR_PODD: u8 = 0x08; // Parity Odd

const UART_LSR_DA: u8 = 0x01; // Data Available
const UART_LSR_OE: u8 = 0x02; // Overrun Error
const UART_LSR_PE: u8 = 0x04; // Parity Error
const UART_LSR_FE: u8 = 0x08; // Framing Error
const UART_LSR_BI: u8 = 0x10; // Break indicator
const UART_LSR_RE: u8 = 0x20; // THR is empty
const UART_LSR_RI: u8 = 0x40; // THR is empty and line is idle
const UART_LSR_EF: u8 = 0x80; // Erroneous data in FIFO

/// Setup the memory required to initialize the UART
pub fn initialize() {
    unsafe {
        ptr::write_volatile(NS16550A_UART.offset(UART_LCR as isize), UART_LCR_DLAB);
        ptr::write_volatile(
            NS16550A_UART.offset(UART_DLL as isize),
            (DIVISOR & 0xff) as u8,
        );
        ptr::write_volatile(
            NS16550A_UART.offset(UART_DLM as isize),
            ((DIVISOR >> 8) & 0xff) as u8,
        );
        ptr::write_volatile(
            NS16550A_UART.offset(UART_LCR as isize),
            UART_LCR_PODD | UART_LCR_8BIT,
        );
    }
}

/// Write a single character
fn putchar(ch: u8) {
    unsafe {
        while (ptr::read_volatile(NS16550A_UART.offset(UART_LSR as isize)) & UART_LSR_RI) == 0 {}
        ptr::write_volatile(NS16550A_UART, ch & 0xff)
    }
}

// TODO(bwb): Writer logic belongs elsewhere
pub struct Writer;

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            putchar(byte);
        }
        Ok(())
    }
}
