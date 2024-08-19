ENTRY(entry)

SECTIONS
{
    . = 0x7c00;
    .text :
    {
        *(.text.entry)
        *(.text .text.*)
    }

    . = 0x7c00 + 510;
    .magic_number :
    {
        SHORT(0xaa55)
    }
}