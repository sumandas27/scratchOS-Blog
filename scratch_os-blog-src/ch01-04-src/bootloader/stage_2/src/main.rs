#![no_std]
#![no_main]

#[no_mangle]
fn stage_2_main() -> !
{
    cmn::btl_print(b"Hello from stage 2!");
    cmn::inf_loop()
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> !
{
    loop {}
}