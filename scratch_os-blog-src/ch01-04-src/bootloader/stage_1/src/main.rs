#![no_std]
#![no_main]
#![feature(naked_functions)]

extern "Rust"
{
    fn stage_2_main() -> !;
}

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

        "push edx",
        "call {main}",

        main = sym stage_1_main,
        options(noreturn)
    )}
}

extern "cdecl" fn stage_1_main(disk_drive: u8) -> !
{   
    let sbb_carry_flag_value: i16;
    unsafe { core::arch::asm!
    (
        "int 19",
        "sbb {sbb_reg:x}, {sbb_reg:x}",
        sbb_reg = out(reg) sbb_carry_flag_value,
        inout("ah") 65u8 => _,
        inout("bx") 0xaa55u16 => _,
        inout("dl") disk_drive => _
    )}

    if sbb_carry_flag_value != 0
    {
        cmn::btl_print(b"BIOS Ext Not Supported");
        cmn::inf_loop();
    }

    const STAGE_2_START: u16 = 0x7e00;
    const STAGE_2_END: u16 = 0x9000;
    const SECTOR_LENGTH: u16 = 512;
    let dap = DiskAddressPacket::new((STAGE_2_END - STAGE_2_START) / SECTOR_LENGTH, STAGE_2_START as u32, 1);

    let load_return_code: u8;
    unsafe { core::arch::asm!
    (
        "push ds",
        "push si",
        "mov si, {addr:x}",
        "int 19",
        "pop si",
        "pop ds",
        addr = inout(reg) &dap as *const _ as u16 => _,
        inout("ah") 66u8 => load_return_code,
        inout("dl") disk_drive => _
    )}

    if load_return_code != 0
    {
        cmn::btl_print(b"Rest of Btl Load Failed");
        cmn::inf_loop();
    }

    unsafe { stage_2_main() }
}

#[repr(C, packed)]
struct DiskAddressPacket
{
    dap_size:               u8,
    always_zero:            u8,
    sectors_to_transfer:    u16,
    ram_start:              u32,
    sector_start:           u64
}

impl DiskAddressPacket
{
    fn new(sectors_to_transfer: u16, ram_start: u32, sector_start: u64) -> Self
    {
        return Self
        {
            dap_size: core::mem::size_of::<Self>() as u8,
            always_zero: 0,
            sectors_to_transfer: sectors_to_transfer,
            ram_start: ram_start,
            sector_start: sector_start
        };
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> !
{
    loop {}
}