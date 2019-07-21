.equ STACK_SHIFT,  10
.equ STACK_SIZE,  1024
.equ MAX_HARTS,  4

.section .text.init,"ax",@progbits

.globl _start
_start:
    # Set up stack pointer based on hartid
    csrr    t0, mhartid
    slli    t0, t0, STACK_SHIFT
    la      sp, stacks + STACK_SIZE
    add     sp, sp, t0

    # Park all harts except hart 0
    csrr    a0, mhartid
    bnez    a0, loop

    # Jump to _rust_start
    j       _rust_start

# No-op loop
loop:
    wfi
    j       loop

    .bss
    .align 4
    .global stacks
stacks:
    .skip STACK_SIZE * MAX_HARTS

