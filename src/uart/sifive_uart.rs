#![allow(unused)]

use core::{fmt, ptr};
use core::convert::TryInto;
use crate::memory_region::MemoryRegion;

/// SIFIVE UART Constants

const UART0: usize = 0x10013000;

struct Uart {
    memory: MemoryRegion
}

impl Uart {
    pub fn new() -> Self {
        let mut memory = unsafe { MemoryRegion::new(UART0, 10000) };
        memory[0x08] = 1;
        memory[0x0C] = 1;
        Uart { memory }
    }

    // TODO error code
    pub fn put_char(&mut self, ch: u8) -> Result<(), ()> {
        self.memory[0x00] = ch as usize;
        Ok(())
    }

    pub fn get_char(&self) -> u8 {
        self.memory[0x04].try_into().unwrap()
    }
}
