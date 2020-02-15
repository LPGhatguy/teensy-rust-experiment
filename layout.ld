MEMORY
{
    FLASH (rx) : ORIGIN = 0x00000000, LENGTH = 1M
    RAM  (rwx) : ORIGIN = 0x1FFF0000, LENGTH = 256K
}

SECTIONS
{
    .text : {
        . = 0;
        KEEP(*(.vectors))
        . = 0x400;
        KEEP(*(.flashconfig*))
        . = ALIGN(4);
        *(.text*)
    } > FLASH = 0xFF

    .rodata : {
        _rodata_start = .;
        *(.rodata*)
        _rodata_end = .;
    } > FLASH

    .bss : {
         . = ORIGIN(RAM);
         _bss_start = .;
         *(.bss*)
         _bss_end = .;
    } > RAM

    .data : {
        _data_start = .;
        *(.data*)
    } > RAM

    _stack_top = ORIGIN(RAM) + LENGTH(RAM);

    /DISCARD/ : {
        *(.ARM.*)
    }
}