ENTRY(_start)
MEMORY { RAM (rwx): ORIGIN = 0x80000, LENGTH = 0x800000 }
SECTIONS {
  .text   : { *(.text*)   } > RAM
  .rodata : { *(.rodata*) } > RAM
  .data   : { *(.data*)   } > RAM
  .bss (NOLOAD) : { *(.bss*) *(COMMON) } > RAM
  _stack_start = ORIGIN(RAM) + LENGTH(RAM);
}
