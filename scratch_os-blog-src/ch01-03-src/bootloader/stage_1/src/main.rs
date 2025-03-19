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
    btl_print(b"Hello, World!");
    inf_loop()
}

fn btl_print_char(char_to_print: u8)
{
    unsafe { core::arch::asm!
    (
        "int 16",
        inout("ah") 14u8 => _,
        inout("al") char_to_print => _
    )}
}

fn btl_print(str_to_print: &[u8])
{
    for &c in str_to_print
    {
        btl_print_char(c);
    }
}

fn inf_loop() -> !
{
    unsafe { core::arch::asm!
    (
        "cli",
        "hlt",
        options(noreturn)
    )}
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> !
{
    loop {}
}