SECTIONS {
    . = 0x80000000 + 0x1000;
    .text : { *(.text) }
}