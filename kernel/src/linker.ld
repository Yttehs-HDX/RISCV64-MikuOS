OUTPUT_ARCH(riscv)
ENTRY(_start)
BASE_ADDR = 0x80200000;

SECTIONS
{
    . = BASE_ADDR;

    .text ALIGN(4) : {
        *.(.text .text.*)
    }

    .rodata ALIGN(4) : {
        *(.rodata .rodata.*)
    }

    .data ALIGN(4) : {
        *(.data .data.*)
    }

    .bss ALIGN(4) : {
        sbss = .;
        *(.bss .bss.*)
        . = ALIGN(4);
        ebss = .;
    }
}