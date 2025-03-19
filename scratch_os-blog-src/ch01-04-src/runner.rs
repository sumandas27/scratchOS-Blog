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

    let root_dir: PathBuf = env::current_dir()?;
    let stage_1_dir: PathBuf = env::current_dir()?.join("bootloader/stage_1");
    let stage_2_dir: PathBuf = env::current_dir()?.join("bootloader/stage_2");

    /* cd to `bootloader/stage_1`
     * ----------------------------------------------------------------------------------------------------------------
     */
    env::set_current_dir(&stage_1_dir)?;

    /* cargo build --release
     * ----------------------------------------------------------------------------------------------------------------
     */
    Command::new("cargo")
        .args(&["build", "--release"])
        .status()?;

    /* cd target/<name_of_your_json_file>/release
     *
     * Within the code reference, I named my target specification JSON file as `x86-16-bit.json`. So in my case I `cd` 
     * into `target/x86-16-bit/release`.
     * ----------------------------------------------------------------------------------------------------------------
     */
    let artifacts_dir: PathBuf = env::current_dir()?.join("target/x86-16-bit/release");
    env::set_current_dir(&artifacts_dir)?;

    /* objcopy -I elf32-i386 -O binary stage_1 stage_1.bin
     * ----------------------------------------------------------------------------------------------------------------
     */
    Command::new(OBJCOPY_STR)
        .args(&["-I", "elf32-i386", "-O", "binary", "stage_1", "stage_1.bin"])
        .status()?;

    /* `stage_1.bin` generation complete
     * cd to `bootloader/stage_2`
     * ----------------------------------------------------------------------------------------------------------------
     */
    env::set_current_dir(&stage_2_dir)?;

     /* cargo build --release
      * ----------------------------------------------------------------------------------------------------------------
      */
    Command::new("cargo")
        .args(&["build", "--release"])
        .status()?;
 
    /* cd target/<name_of_your_json_file>/release
     * ----------------------------------------------------------------------------------------------------------------
     */
    let artifacts_dir: PathBuf = env::current_dir()?.join("target/x86-16-bit/release");
    env::set_current_dir(&artifacts_dir)?;

    /* objcopy -I elf32-i386 -O binary stage_2 stage_2.bin
     * ----------------------------------------------------------------------------------------------------------------
     */
    Command::new(OBJCOPY_STR)
        .args(&["-I", "elf32-i386", "-O", "binary", "stage_2", "stage_2.bin"])
        .status()?;

    /* `stage_2.bin` generation complete
     * cd back to root directory 
     * ----------------------------------------------------------------------------------------------------------------
     */
    env::set_current_dir(&root_dir)?;

    /* create an empty at `<path to your disk image>/disk_image.iso` file
     * i decide to place mine in a new top-level `disk_img` directory
     * 
     * i first create the `disk_img` directory if it doesn't already exist.
     * ----------------------------------------------------------------------------------------------------------------
     */
    std::fs::create_dir_all("disk_img/")?;
    let mut disk_image_iso = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open("disk_img/disk_image.iso")?;

    // create a handle to `stage_1.bin`
    let mut stage_1_bin = std::fs::File::open("bootloader/stage_1/target/x86-16-bit/release/stage_1.bin")?;

    // create a handle to `stage_2.bin`
    let mut stage_2_bin = std::fs::File::open("bootloader/stage_2/target/x86-16-bit/release/stage_2.bin")?;

    // append `stage_1.bin` to `disk_image.iso`
    std::io::copy(&mut stage_1_bin, &mut disk_image_iso)?;

    // append `stage_2.bin` to `disk_image.iso`
    std::io::copy(&mut stage_2_bin, &mut disk_image_iso)?;

    /* qemu-system-x86_64 -drive format=raw,file=disk_img/disk_image.iso
     * ----------------------------------------------------------------------------------------------------------------
     */
    Command::new("qemu-system-x86_64")
        .args(&["-drive", "format=raw,file=disk_img/disk_image.iso"])
        .status()?;

    return Ok(());
}