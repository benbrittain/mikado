//! Core-Local Interruptor

use crate::memory_region::MemoryRegion;
use crate::{print, println};

pub struct Clint {
    memory: MemoryRegion,
}

impl Clint {
    pub fn new() -> Self {
        Clint {
            memory: unsafe { MemoryRegion::new(0x2000000, 0xc000) },
        }
    }

    pub fn get_time_cmp(&self) -> usize {
        self.memory[0x4000]
    }

    pub fn set_time_cmp(&mut self, time: usize) {
        self.memory[0x4000] = time;
    }

    pub fn get_time(&self) -> usize {
        self.memory[0xbff8]
    }
}

#[test_case]
fn blah() {
    print!("Clint creation... ");
    let clint = Clint::new();
    println!("[ok]");
}
