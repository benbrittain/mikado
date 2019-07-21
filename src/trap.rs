use crate::println;

#[no_mangle]
pub fn m_trap_handler(registers: *mut usize, mcause: usize, mepc: usize) {
    println!(
        "registers: {:?} | mcause: {} | mepc: {:x} ",
        registers, mcause, mepc
    );

    // `m_trap_vector` calls this function. It ends with a `mret`.
    // `mepc` needs to be incremented so the trap handling can make progress
    unsafe { asm!("csrw $0, $1" :: "i"(0x341), "r"(mepc + 4)) }
}
