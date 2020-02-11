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
        *(.rodata*)
    } > FLASH

    _stack_top = ORIGIN(RAM) + LENGTH(RAM);

    /DISCARD/ : {
        *(.ARM.*)
    }
}