//! Platform-Level Interrupt Controller

use crate::memory_region::MemoryRegion;

pub struct Plic {
    memory: MemoryRegion,
}

pub enum Priority {
    Disable = 0,
    One = 1, // low
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7, // high
}

impl Plic {
    pub fn new() -> Self {
        Plic {
            memory: unsafe { MemoryRegion::new(0x0C00_0000, (0x1000_0000 - 0x0C00_0000)) },
        }
    }

    pub fn get_source_priority(&self, interrupt: u16) -> usize {
        self.memory[4 * (interrupt as u32)]
    }

    pub fn set_source_priority(&mut self, interrupt: u16, priority: Priority) {
        self.memory[4 * (interrupt as u32)] = (priority as usize)
    }

    pub fn claim(&self) -> usize {
        self.memory[0x0C20_0004 - 0x0C00_0000]
    }

    //pub fn set_time_cmp(&mut self, time: usize) {
    //    self.memory[0x4000] = time;
    //}

    //pub fn get_time(&self) -> usize {
    //    self.memory[0xbff8]
    //}
}

//#[test_case]
//fn blah() {
//    print!("Clint creation... ");
//    let clint = Clint::new();
//    println!("[ok]");
//}
