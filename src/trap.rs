use crate::println;
use crate::riscv::csr;

#[no_mangle]
pub fn m_trap_handler(registers: *mut usize, mcause: usize, mepc: usize) {
    println!(
        "registers: {:?} | mcause: {} | mepc: {:x} ",
        registers, mcause, mepc
    );

    // `m_trap_vector` calls this function. It ends with a `mret`.
    // `mepc` needs to be incremented so the trap handling can make progress
    unsafe {
        csr::write_mepc(mepc + 4);
    }
}
