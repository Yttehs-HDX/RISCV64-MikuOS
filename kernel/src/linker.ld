OUTPUT_ARCH(riscv)
ENTRY(_start)

SBI_ENTRY_POINT = 0x80200000;
BASE_ADDR = 0xffffffffc0200000; /* 0xffffffffc0000000 + 2000000 */

SECTIONS {
    . = BASE_ADDR;

    /* kernel start */
    skernel = .;

    /* text start */
    stext = .;
    .text : AT(SBI_ENTRY_POINT) {
        *(.text.entry)
        . = ALIGN(4K);
        *(.text.trap)
        *(.text .text.*)
        *(.stext .stext.*)
    }
    . = ALIGN(4K);
    etext = .;
    /* text end */

    /* rodata start */
    srodata = .;
    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    }
    . = ALIGN(4K);
    erodata = .;
    /* rodata end */

    /* data start */
    sdata = .;
    .data : {
        *(.data.root_page)
        *(.data .data.*)
        *(.sdata .sdata.*)
    }
    . = ALIGN(4K);
    edata = .;
    /* data end */

    /* bss start */
    sbss = .;
    .bss : {
        *(.bss.boot_stack)
        sbss_no_stack = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
    }
    . = ALIGN(4K);
    ebss = .;
    /* bss end */

    ekernel = .;
    /* kernel end */

    /DISCARD/ : {
        *(.eh_frame)
    }
}