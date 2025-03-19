#![no_std]

/* This function requires BIOS to run, which only exists on the x86 architecture.
 */
#[cfg(target_arch = "x86")]
pub fn btl_print(str_to_print: &[u8])
{
    for &c in str_to_print
    {
        btl_print_char(c);
    }
}

/* This function is valid on both x86 and x86_64 architectures.
 */
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub fn inf_loop() -> !
{
    unsafe { core::arch::asm!
    (
        "cli",
        "hlt",
        options(noreturn)
    )}
}

/* This function requires BIOS to run, which only exists on the x86 architecture.
 */
#[cfg(target_arch = "x86")]
fn btl_print_char(char_to_print: u8)
{
    unsafe { core::arch::asm!
    (
        "int 16",
        inout("ah") 14u8 => _,
        inout("al") char_to_print => _
    )}
}