use std::{env, path::PathBuf};
use std::process::{Stdio, Command};

fn main() -> std::io::Result<()>
{
    /* check `objcopy` dependency:
     * -----------------------------------------------------------------------------------------------------------------
     */
    const OBJCOPY_STR: &'static str = if cfg!(target_os = "macos") { "x86_64-elf-objcopy" } else { "objcopy"};
    let objcopy_res = Command::new(OBJCOPY_STR)
        .arg("--version")
        .stdout(Stdio::null())
        .status();
        
    if objcopy_res.is_err()
    {
        panic!
        (
            "{}",
            "\'objcopy\' could not be found on your machine:\n\
             - On macOS: Run \'brew install x86_64-elf-binutils\'\n\
             - On Windows: Run \'pacman -S mingw-w64-x86_64-binutils\' and make sure \'C:\\mysys64\\mingw64\\bin\' \
               is part of your System's \'Path\' environment variable."
        );
    }

    /* check QEMU Emulator dependency:
     * -----------------------------------------------------------------------------------------------------------------
     */
    let qemu_emulator_res = Command::new("qemu-system-x86_64")
        .arg("--version")
        .stdout(Stdio::null())
        .status();

    if qemu_emulator_res.is_err()
    {
        panic!
        (
            "{}",
            "The QEMU Emulator isn't installed on your machine. Installation instructions can be found at \
             \'https://www.qemu.org/download/\'"
        );
    }

    /* 1. `cd bootloader/stage_1`
     * ----------------------------------------------------------------------------------------------------------------
     */
    let stage_1_dir: PathBuf = env::current_dir()?.join("bootloader/stage_1");
    env::set_current_dir(&stage_1_dir)?;

    /* 2. `cargo build --release`
     * ----------------------------------------------------------------------------------------------------------------
     */
    Command::new("cargo")
        .args(&["build", "--release"])
        .status()?;

    /* 3. `cd target/<name_of_your_json_file>/release`
     *
     * Within the code reference, I named my target specification JSON file as `x86-16-bit.json`. So in my case I `cd` 
     * into `target/x86-16-bit/release`.
     * ----------------------------------------------------------------------------------------------------------------
     */
    let artifacts_dir: PathBuf = env::current_dir()?.join("target/x86-16-bit/release");
    env::set_current_dir(&artifacts_dir)?;

    /* 4. `objcopy -I elf32-i386 -O binary stage_1 stage_1.bin`
     * ----------------------------------------------------------------------------------------------------------------
     */
    Command::new(OBJCOPY_STR)
        .args(&["-I", "elf32-i386", "-O", "binary", "stage_1", "stage_1.bin"])
        .status()?;

    /* 5. `qemu-system-x86_64 -drive format=raw,file=stage_1.bin`
     * ----------------------------------------------------------------------------------------------------------------
     */
    Command::new("qemu-system-x86_64")
        .args(&["-drive", "format=raw,file=stage_1.bin"])
        .status()?;

    return Ok(());
}