ENTRY(stage_2_main)

SECTIONS
{
    . = 0x7e00;
    .text :
    {
        *(.text.stage_2_main)
        *(.text .text.*)
    }

    .rodata :
    {
        *(.rodata .rodata.*)
    }

    . = 0x9000 - 2;
    .stage_2_end_bytes :
    {
        SHORT(0x0000)
    }
}