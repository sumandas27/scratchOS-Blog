#![no_std]
#![no_main]
#![feature(naked_functions)]

#[naked]
#[no_mangle]
extern "C" fn entry() -> !
{
    unsafe { core::arch::asm!
    (
        "mov sp, 0x7c00",

        "mov ax, 0",
        "mov ds, ax",
        "mov es, ax",
        "mov fs, ax",
        "mov gs, ax",
        "mov ss, ax",

        "call {main}",

        main = sym stage_1_main,
        options(noreturn)
    )}
}

fn stage_1_main() -> !
{
    print_char(b'H');
    print_char(b'e');
    print_char(b'l');
    print_char(b'l');
    print_char(b'o');
    print_char(b',');
    print_char(b' ');
    print_char(b'W');
    print_char(b'o');
    print_char(b'r');
    print_char(b'l');
    print_char(b'd');
    print_char(b'!');
    
    loop {}
}

fn print_char(char_to_print: u8)
{
    unsafe { core::arch::asm!
    (
        "int 16",
        inout("ah") 14u8 => _,
        inout("al") char_to_print => _
    )}
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> !
{
    loop {}
}