# Writing a Boot Sector

We have a freestanding Rust binary designed for the x86 architecture, but it's still not quite ready yet for an x86 machine on start-up.

## Creating a Raw Binary File

An x86 machine on start-up expects a *raw binary file* as a bootloader. Let's see the format of our current executable...

> /🐧 **On Mac/Linux:** The `file` command allows us to see the format of any specified file.
>
> ⊞ **On Windows:** The `file` command on Windows is accessible inside the MSYS2 Shell which can be installed through <https://www.msys2.org/>. This shell will be needed for installing future command line tools as well.

<p align="center">
  <img width="500px" src="img/stage-1-file-cmd.png">
</p>

The Rust compiler generated a file in the *ELF format*, which is essentially a binary file with a bunch of associated metadata. An x86 machine on start-up is *not* expecting any of this metadata when it runs our program, so our task is now to strip it from our generated file.

Thankfully, there is a command line tool called `objcopy` that precisely strips metadata from ELF files.

> 🐧 **On Linux:** `objcopy` is installed on Linux machines by default.
>
>  **On Mac:** Running `brew install x86_64-elf-binutils` allows you to use the `x86_64-elf-objcopy` tool, which is the same as Window's and Linux's `objcopy`.
>
> ⊞ **On Windows:**
> 1. If you haven't already, install the MSYS2 Shell through <https://www.msys2.org/>.
> 2. Running `pacman -S mingw-w64-x86_64-binutils` within the MSYS2 Shell installs `objcopy` on your machine.
> 3. If you still cannot run `objcopy`, make sure that `C:\mysys64\mingw64\bin` (MSYS2 Shell's binaries directory) is part of your system's `Path` environment variable.
>     1. Search up `environment variabes`, open the `Control panel` and choose <kbd>Environment Variables...</kbd>
>     2. Under `System variables`, add `C:\mysys64\mingw64\bin` to the `Path` variable. Restart your shell and `objcopy` should be usable now.
>
> You can verify installation by running `objcopy --version` on Linux and Windows or `x86_64-elf-objcopy` on the Mac.

The `objcopy` command has the following format:

```properties
objcopy -I <input-format> -O <output-format> <input-file> <output-location>
```

Let's go through each of these fields:

* The specific `<input-format>` of the generated file is `elf32-i386`, and we can see this matches with what command-line tools say the format of the file is:

<p align="center">
  <img width="500px" src="img/elf32-i386.png">
</p>

* We want the `<output-format>` to be a raw `binary` file.
* The `<input-file>` we want to convert is the `stage_1` compiled file.
* I'll call the `<output-location>` as `stage_1.bin`, emphasizing that this file is a *raw binary file*.

The final command to run (within the directory with your compiled `stage_1` file) is:

```properties
objcopy -I elf32-i386 -O binary stage_1 stage_1.bin
```

> 🚨 **REMINDER:** If you are on a Mac , remember to write `x86_64-elf-objcopy` instead of just `objcopy`.

We finally have a file, `stage_1.bin`, in the format that an x86 machine on start-up would like. Let's run it.

## The QEMU Emulator

Testing your bootloader and your operating system on an actual computer is difficult as every time any change is made, you need to burn your project onto a USB stick and reboot your machine. If you develop with only one machine, constantly switching and rebooting between the operating system your making and the operating system you're developing on will really slow down any progress you're making. 

Instead, we are going to use an *emulator*, which emulates the hardware of a regular computer. This allows you to develop and test on one machine without needing constant reboots.

`QEMU` is definitely the most popular emulator used for OS development. You can find installation instructions at <https://www.qemu.org/download/>.

Installing `QEMU` should allow you to use the `qemu-system-x86_64` command, which emulates an x86 machine. Run `qemu-system-x86_64 --version` on your terminal to verify that it's there. This is nice because after we are done with setup, it won't matter which architecture your machine uses as we will all be developing for x86 through the `QEMU`'s x86 emulator.

We can run our generated `stage_1.bin` file on QEMU with the command...

```properties
qemu-system-x86_64 -drive format=raw,file=stage_1.bin
```

After a few seconds, QEMU will complain that there is `No bootable device`.

<p align="center">
  <img width="500px" src="img/qemu-no-bootable-device.png">
</p>

QEMU isn't recognizing our binary file as valid bootable code. Let's revisit exactly how QEMU and other x86 machines on start-up handle the booting process.

## What Went Wrong? Revisiting the Booting Process

The command tells QEMU to treat our binary file `stage_1.bin` as its emulated *hard disk*, which is a type of secondary storage.

> **A Quick Computer Architecture Tour**
>
> A computer's *central processing unit* (CPU) is the hardware computer that is responsible for running computer instructions.
>
> <p align="center">
>   <img width="400px" src="img/quick-computer-architecture.png">
> </p>
>
> However, running instructions isn't useful if you don't have memory to work with. *Random Access Memory* (RAM) is a form of *primary memory storage* that a CPU can directly work with. 
>
> *Secondary memory storages* (like the *hard disk*) are usually significantly larger than primary storage, but cannot directly be used by the CPU. Memory from secondary storage must first be copied over to primary storage before the CPU can work on it.
>
> A computer's primary storage is generally wiped and reset every time a computer is powered on. Memory in secondary storage, on the other hand, persists throughout computer start-ups. This is why bootloaders and kernels are kept in secondary storage, so they don't get wiped and deleted in primary storage on a power off.

Secondary storage devices like the hard disk are split into *sectors*, which are 512 bytes long. If the *first* sector of a hard disk has the magic number `0xaa55` as its last 2 bytes (511<sup>th</sup> and 512<sup>th</sup> bytes), then the entire hard disk is considered bootable.

<p align="center">
  <img width="500px" src="img/a-bootable-hard-disk.png">
</p>

> ❓ **What is `0xaa55`?**
>
> `0xaa55` is a number where the `0x` prefix declares the rest of the number to be in *base-16*, or in *hexadecimal*, which has 16 digits (`0` - `9` and `a` - `f`) instead of the decimal system's 10 digits. 
>
> Some other common prefixes:
> * `0b` prefixes a number in binary (*base-2*, with only `0` and `1` as its digits)
> * `0o` prefixes a number in octal (*base-8*, with digits `0` - `7`)

① **We don't have `0xaa55` as the 511<sup>th</sup> and 512<sup>th</sup> bytes of our program's raw binary file**, which is why it won't boot.

Remember, memory in secondary storage isn't directly usable by the CPU. In order for the CPU to start operating on our bootloader in its hard disk, it copies the first sector in the hard disk and pastes it into RAM (primary storage) *specifically* at memory location `0x7c00`. It will then interpret the start of the first sector at `0x7c00` as computer instructions and start running them on the CPU.

If we want the machine to execute our program, ② **we need to put our program at the *start* of the hard disk**. This allows the program to be placed at `0x7c00` in RAM where the CPU will begin executing it.

## Creating a Linker Script

To recap, we have 2 things to fix:

1. Place `0xaa55` at the end of our program raw binary's first sector.
2. Place the program instructions themselves at the *start* of the raw binary.

Unfortunately, this cannot be done directly within Rust code. We are going to need the help of a *linker script*, which acts as a recipe that Rust's linker follows to generate the raw binary file.

Rust auto-generates a linker script by default, but we can override it by adding `build.rs` within the `stage_1` directory. The `build.rs` file is a *build script*, which allows you to set configuration properties before the project is compiled.

```Rust
// build.rs

fn main()
{
    println!("cargo:rustc-link-arg-bins=--script=linker.ld");
}
```

This build script tells us to use `linker.ld` as the linker script (rather than the generated default one). This file doesn't exist just yet, so let's add it:

```
stage_1
├── .cargo
│   └── config.toml
├── Cargo.toml
├── build.rs
├── linker.ld
└── src
    └── main.rs
```

`build.rs` and `linker.ld` are the newly added files.

The linker script has two components:

1. One defining the *entry point* of the program.
2. The other defining the *sections* of your program.

The skeleton of a linker scipt looks like this:

```Linker Script
ENTRY(①)

SECTIONS
{
    ②
}
```

① is where we will be declaring our entry point, and ② is where we define the ordering of our sections.

## Creating an Entry Point

Let's start by adding an entry point to our Rust program. We'll just have it infinitely looping for now:

```Rust
#![no_std]
#![no_main]

#[no_mangle]
fn entry() -> !
{
    loop {}
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo)
{
    loop {}
}
```

...and now we update the linker script:

```Linker Script
ENTRY(entry)

SECTIONS
{
    ②
}
```

> ❓ **What is `#[no_mangle]` and why is it needed?**
>
> Rust allows multiple functions to have the same name, as long as they are in *different scopes*. For example, the following code is valid:
>
> ```Rust
> struct A;
> struct B;
>
> impl A { fn foo() { ... } }
> impl B { fn foo() { ... } }
> ```
>
> Notice how there are two functions both named `foo`, but in different scopes.
>
> However, functions *cannot* have duplicate names by the time they are sent to Rust's linker. So, the compiler "mangles" these function names into some encoded (but unique) mess to be sent to the linker. For example, `A::foo()` will be mangled into something like...
>
> ```
> _ZN7example3A3foo17h16fcc82fa6043ccbE
> ```
>
> If the `entry` function doesn't have the `#[no_mangle]` attribute, then the label we declared in the linker script (`entry`) won't match up with the mangled label in Rust, and the linker wouldn't be able to find the function our linker script refers to.
>
> So, we tell the the compiler to leave the `entry` function unmangled with the `#[no_mangle] attribute so that the linker script and the linker are on the same page.

## Placing the Entry Point

We now have an `entry` point, but program entry points are not necessarily placed in the beginning of the produced binary. For example, Rust and C have no specification in the ordering of function placements, meaning their `main` entry point can be who knows where?

This is where the `SECTIONS` part of the linker script, which is where we get to define the ordering of the different *link sections* of the program