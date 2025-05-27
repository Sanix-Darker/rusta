ENTRY(_start)
MEMORY {
    RAM (rwx): ORIGIN = 0x80000, LENGTH = 0x800000
}

SECTIONS {
    .text : {
        KEEP(*(.text.boot))
        *(.text .text.*)
    } > RAM

    .rodata : ALIGN(4) {
        *(.rodata .rodata.*)
    } > RAM

    .data : ALIGN(4) {
        *(.data .data.*)
    } > RAM

    .bss (NOLOAD) : ALIGN(4) {
        __bss_start = .;
        *(.bss .bss.*)
        *(COMMON)
        __bss_end = .;
    } > RAM

    .stack (NOLOAD) : ALIGN(8) {
        __stack_start = .;
        . += 0x4000; /* 16KB stack */
        __stack_end = .;
    } > RAM

    _end = .;
}
