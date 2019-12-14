/// Control and Status Registers
///
/// Documentation Comments are derivative works of
/// "The RISC-V Instruction Set Manual Volume II: Privileged Architecture
/// Document Version 20190608-Priv-MSU-Ratified"
///
/// Creative Commons Attribution 4.0 International License.
use core::convert::TryInto;

/// The Wait for Interrupt instruction (WFI) provides a hint to the implementation that the current
/// hart can be stalled until an interrupt might need servicing. Execution of the WFI instruction
/// can also be used to inform the hardware platform that suitable interrupts should preferentially be
/// routed to this hart. WFI is available in all privileged modes, and optionally available to U-mode.
pub fn wfi() {
    unsafe {
        asm!("wfi" ::: "memory");
    }
}



#[repr(u16)]
#[allow(dead_code)]
pub enum Register {
    Fflags = 0x001,
    Frm = 0x002,
    Fcsr = 0x003,
    Mcycle = 0xB00,
    Minstret = 0xB02,
    Mcycleh = 0xB80,
    Minstreth = 0xB82,
    Cycle = 0xC00,
    Time = 0xC01,
    Instret = 0xC02,
    Cycleh = 0xC80,
    Timeh = 0xC81,
    Instreth = 0xC82,
    Mvendorid = 0xF11,
    Marchid = 0xF12,
    Mimpid = 0xF13,
    Mhartid = 0xF14,
    Mstatus = 0x300,
    Misa = 0x301,
    Medeleg = 0x302,
    Mideleg = 0x303,
    Mie = 0x304,
    Mtvec = 0x305,
    Mcounteren = 0x306,
    Mscratch = 0x340,
    Mepc = 0x341,
    Mcause = 0x342,
    Mtval = 0x343,
    Mip = 0x344,
    Sstatus = 0x100,
    Sedeleg = 0x102,
    Sideleg = 0x103,
    Sie = 0x104,
    Stvec = 0x105,
    Scounteren = 0x106,
    Sscratch = 0x140,
    Sepc = 0x141,
    Scause = 0x142,
    Stval = 0x143,
    Sip = 0x144,
    Satp = 0x180,
    Pmpcfg0 = 0x3A0,
    Pmpcfg1 = 0x3A1,
    Pmpcfg2 = 0x3A2,
    Pmpcfg3 = 0x3A3,
    Pmpaddr0 = 0x3B0,
    Pmpaddr1 = 0x3B1,
    Pmpaddr2 = 0x3B2,
    Pmpaddr3 = 0x3B3,
    Pmpaddr4 = 0x3B4,
    Pmpaddr5 = 0x3B5,
    Pmpaddr6 = 0x3B6,
    Pmpaddr7 = 0x3B7,
    Pmpaddr8 = 0x3B8,
    Pmpaddr9 = 0x3B9,
    Pmpaddr10 = 0x3BA,
    Pmpaddr11 = 0x3BB,
    Pmpaddr12 = 0x3BC,
    Pmpaddr13 = 0x3BD,
    Pmpaddr14 = 0x3BE,
    Pmpaddr15 = 0x3BF,
}

/// Coerce Register into u16.
/// This is a workaround for https://github.com/rust-lang/rust/issues/42974
/// TODO: remove need for this macro and consume enum directly
macro_rules! reg {
    ( $reg:expr ) => {
        $reg as u16
    }
}

/// Write to an arbitrary Control and Status Register
///
/// Warning:
/// This currently does not work due to constant folding and the
/// inline assembly constraints on 'i'
#[macro_export]
macro_rules! write_csr {
    ( $reg:expr, $value:expr ) => {
        #[allow(unused_unsafe)]
        unsafe { asm!("csrw $0, $1" :: "i"($reg), "r"($value)) }
    }
}

/// The mepc CSR is a WARL register that must be able to hold all valid physical and virtual addresses.
///
/// When a trap is taken into M-mode, mepc is written with the virtual address of
/// the instruction that was interrupted or that encountered the exception.
///
/// No guarantee the virtual address is valid, so unsafe
pub unsafe fn write_mepc(value: usize) {
    write_csr!(reg!(Register::Mepc), value);
}

/// Read from an arbitrary Control and Status Register
#[macro_export]
macro_rules! read_csr {
    ( $r:expr ) => {
        {
            let result: usize;
            unsafe { asm!("csrr $0, $1" : "=r"(result) : "i"($r)) }
            result
        }
    }
}

/// The misa CSR is a WARL read-write register reporting the ISA supported by the hart.
pub fn misa() -> usize {
    let result;
    unsafe { asm!("csrr $0, $1" : "=r"(result) : "i"(reg!(Register::Misa))) }
    result
}

/// The mcycle CSR counts the number of clock cycles executed by the processor core on which the hart is running.
pub fn mcycle() -> usize {
    let result;
    unsafe { asm!("csrr $0, $1" : "=r"(result) : "i"(reg!(Register::Mcycle))) }
    result
}

use bitfield::*;

pub fn enable_mie() {
    let value: usize = 1 << 3;
    write_csr!(reg!(Register::Mstatus), value);
}

pub fn set_mie(msie: bool, mtie: bool, meie: bool) {
    let mut value: usize = 0;
    if msie {
        value += 1 << 3;
    }
    if mtie {
        value += 1 << 7;
    }
    if meie {
        value += 1 << 11;
    }
    write_csr!(reg!(Register::Mie), value);
}

pub fn mstatus() -> MStatus {
    let result: u32;
    unsafe { asm!("csrr $0, $1" : "=r"(result) : "i"(reg!(Register::Mstatus))) }
    let mut status = MStatus::new();
    // TODO fill in other parts of mstatus. ONLY MIE implemented currentl
    status.set_mie((result == (1 << 3)) as u8);
    status
}

#[bitfield]
pub struct MStatus {
    uie: B1,
    sie: B1,
    wpri_1: B1,
    mie: B1,
    upie: B1,
    spie: B1,
    wpri_2: B1,
    mpie: B1,
    spp: B1,
    wpri_3: B2,
    mpp: B2,
    fs: B2,
    xs: B2,
    mprv: B1,
    sum: B1,
    mxr: B1,
    tvm: B1,
    tw: B1,
    tsr: B1,
    wpri_4: B8,
    sd: B1,
}
