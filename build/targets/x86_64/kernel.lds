ENTRY(kmain)
OUTPUT_FORMAT(elf64-x86-64)

KERNEL_OFFSET = 0xFFFFFF0000000000;

SECTIONS {
    . = KERNEL_OFFSET;

    . += SIZEOF_HEADERS;
    . = ALIGN(4096);

    .text : AT(ADDR(.text) - KERNEL_OFFSET) {
        TEXT_START = .;
        *(.text*)
        . = ALIGN(4096);
        TEXT_END = .;
    }

    .rodata : AT(ADDR(.rodata) - KERNEL_OFFSET) {
        RODATA_START = .;
        *(.rodata*)
        . = ALIGN(4096);
        RODATA_END = .;
    }

    .data : AT(ADDR(.data) - KERNEL_OFFSET) {
        DATA_START = .;
        *(.data*)
        . = ALIGN(4096);
        DATA_END = .;
        BSS_START = .;
        *(.bss*)
        . = ALIGN(4096);
        BSS_END = .;
    }

    .tdata : AT(ADDR(.tdata) - KERNEL_OFFSET) {
        TDATA_START = .;
        *(.tdata*)
        . = ALIGN(4096);
        TDATA_END = .;
        TBSS_START = .;
        *(.tbss*)
        . += 8;
        . = ALIGN(4096);
        TBSS_END = .;
    }

    END = .;

    /DISCARD/ : {
        *(.comment*)
        *(.eh_frame*)
        *(.gcc_except_table*)
        *(.note*)
        *(.rel.eh_frame*)
    }
}