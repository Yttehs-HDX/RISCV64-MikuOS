OUTPUT_ARCH(riscv)
ENTRY(_start)
BASE_ADDR = 0x10000;

SECTIONS {
    # kernel start
    . = BASE_ADDR;
    skernel = .;

    # text start
    stext = .;
    .text : {
        *(.text.entry)
        *(.text .text.*)
        *(.stext .stext.*)
    }

    . = ALIGN(4K);
    etext = .;
    # text end

    # rodata start
    srodata = .;
    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    }

    . = ALIGN(4K);
    erodata = .;
    # rodata end

    # data start
    sdata = .;
    .data : {
        *(.data .data.*)
        *(.sdata .sdata.*)
    }

    . = ALIGN(4K);
    edata = .;
    # data end

    .bss : {
        sbss_with_stack = .;
        *(.bss.stack)

    # bss start
        sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
    }

    . = ALIGN(4K);
    ebss = .;
    # bss end

    ekernel = .;
    # kernel end

    /DISCARD/ : {
        *(.eh_frame)
    }
}