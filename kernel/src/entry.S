    .section .text.entry
    .globl _start
_start:
    la sp, boot_stack_top
    call rust_main

    .section .bss.stack
    .globl boot_stack_lower_bound
    .globl boot_stack_top
boot_stack_lower_bound:
    .space 0x40000000 # 128 MB
boot_stack_top: