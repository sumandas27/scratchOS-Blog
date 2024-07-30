#![no_std]
#![no_main]

#[no_mangle]
fn entry() -> !
{
    loop {}
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> !
{
    loop {}
}