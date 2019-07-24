/// Control and Status Registers
///
/// Documentation Comments are derivative works of
/// "The RISC-V Instruction Set Manual Volume II: Privileged Architecture
/// Document Version 20190608-Priv-MSU-Ratified"
///
/// Creative Commons Attribution 4.0 International License.
pub mod csr {
    // TODO(bwb) Break into mode level modules?
    // might be some interesting type safety if zsts?

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

    // WRITES BELOW

    /// Write to an arbitrary Control and Status Register
    ///
    /// Warning:
    /// This currently does not work due to constant folding and the
    /// inline assembly constraints on 'i'
    #[allow(dead_code)] // TODO make all other writes use this
    pub fn write_csr(reg: Register, value: usize) {
        unsafe { asm!("csrw $0, $1" :: "i"(reg), "r"(value)) }
    }

    /// The mepc CSR is a WARL register that must be able to hold all valid physical and virtual addresses.
    ///
    /// When a trap is taken into M-mode, mepc is written with the virtual address of
    /// the instruction that was interrupted or that encountered the exception.
    ///
    /// No guarantee the virtual address is valid, so unsafe
    pub unsafe fn write_mepc(value: usize) {
        // TODO(bwb): use write_csr
        asm!("csrw $0, $1" :: "i"(Register::Mepc), "r"(value))
    }

    // READS BELOW

    /// Read from an arbitrary Control and Status Register
    pub fn read_csr(r: Register) -> usize {
        let result;
        unsafe { asm!("csrr $0, $1" : "=r"(result) : "i"(r)) }
        result
    }

    /// The misa CSR is a WARL read-write register reporting the ISA supported by the hart.
    pub fn misa() -> usize {
        let result = read_csr(Register::Misa);
        result
    }

}
