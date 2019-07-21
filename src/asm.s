.equ STACK_SHIFT,  10
.equ STACK_SIZE,  1024
.equ MAX_HARTS,  4

# 32bit
.equ REGBYTES, 4
.equ SAVE_REGS, 16
.equ CONTEXT_SIZE, (SAVE_REGS * REGBYTES)

.macro sxsp a, b
sw \a, ((\b)*REGBYTES)(sp)
.endm

.macro lxsp a, b
lw \a, ((\b)*REGBYTES)(sp)
.endm


.section .text.init,"ax",@progbits

.globl _start
_start:
    # Setup trap functionality
    la      t0, m_trap_vector
    csrw    mtvec, t0

    # Setup stack pointer based on hartid
    csrr    t0, mhartid
    slli    t0, t0, STACK_SHIFT
    la      sp, stacks + STACK_SIZE
    add     sp, sp, t0

    # Park all harts except hart 0
    csrr    a0, mhartid
    bnez    a0, loop

    # Jump to _rust_start
    j       _rust_start


.align 2
m_trap_vector:
    # Save registers
    addi    sp, sp, -CONTEXT_SIZE
    sxsp    ra, 0
    sxsp    a0, 1
    sxsp    a1, 2
    sxsp    a2, 3
    sxsp    a3, 4
    sxsp    a4, 5
    sxsp    a5, 6
    sxsp    a6, 7
    sxsp    a7, 8
    sxsp    t0, 9
    sxsp    t1, 10
    sxsp    t2, 11
    sxsp    t3, 12
    sxsp    t4, 13
    sxsp    t5, 14
    sxsp    t6, 15

    # Invoke the M-Mode handler
    mv      a0, sp
    csrr    a1, mcause
    csrr    a2, mepc
    jal     m_trap_handler

    # Restore registers
    lxsp    ra, 0
    lxsp    a0, 1
    lxsp    a1, 2
    lxsp    a2, 3
    lxsp    a3, 4
    lxsp    a4, 5
    lxsp    a5, 6
    lxsp    a6, 7
    lxsp    a7, 8
    lxsp    t0, 9
    lxsp    t1, 10
    lxsp    t2, 11
    lxsp    t3, 12
    lxsp    t4, 13
    lxsp    t5, 14
    lxsp    t6, 15
    addi sp, sp, CONTEXT_SIZE

    # Return from M-Mode trap
    mret

# No-op loop
loop:
    wfi
    j       loop

.bss
.align 4
.global stacks
stacks:
    .skip STACK_SIZE * MAX_HARTS

