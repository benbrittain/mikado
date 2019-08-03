use core::mem;
use core::ops::{Index, IndexMut};

pub struct MemoryRegion {
    ptr: *mut usize,
    length_bytes: usize,
}

impl MemoryRegion {
    pub unsafe fn new(address: usize, length: usize) -> Self {
        assert_eq!(length % mem::size_of::<usize>() as usize, 0);
        Self {
            ptr: address as *mut usize,
            length_bytes: length,
        }
    }
}

impl Index<u32> for MemoryRegion {
    type Output = usize;
    fn index(&self, index: u32) -> &usize {
        assert_eq!(index as usize % mem::size_of::<usize>() as usize, 0);
        unsafe { &*(self.ptr.add(index as usize / mem::size_of::<usize>())) }
    }
}

impl IndexMut<u32> for MemoryRegion {
    fn index_mut(&mut self, index: u32) -> &mut usize {
        assert_eq!(index % mem::size_of::<usize>() as u32, 0);
        unsafe { &mut *(self.ptr.add(index as usize / mem::size_of::<usize>())) }
    }
}
