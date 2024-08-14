# Exploring the Machine's Environment

The start of our bootloader code is going to pretty involved and will require a lot of inline assembly, so I don't think it's smart to jump straight into the code. Instead, I'll spend a lot of this section exploring the environment we are dropped into after the machine loads the first disk image sector into RAM to motivate what our bootloader code aims to do. We'll wrap up this section by printing a `Hello, World!` from our bootloader.

One thing before we start, running the commands to generate and run our raw binary over and over again is a bit tedious, so my recommendation is to find some way to automate it. A shell script at the top-level directory `scratch_os/` would look like...

```properties
cd bootloader/stage_1/
cargo run
cd target/x86-16-bit/debug/
objcopy -I elf32-i386 -O binary stage_1 stage_1.bin
qemu-system-x86_64 -drive format=raw,file=stage_1.bin
```

>  Macs would use `x86_64-elf-objcopy` instead of `objcopy`.

You could write a `Makefile` for this on Unix-based systems, although they aren't trivial to learn/write. 

I keep my automated binary generation code in Rust, and make the top-level directory `scratch_os/` into its own `cargo` project. That way, all I have to do is run `cargo run` from the top-level directory `scratch_os`, and the final raw binary is generated and executed on QEMU.

> ℹ️ My source code reference for the above project structure can be found under `scratch_os-blog-src/ch01-03-src` within this blog's [GitHub Repository](https://github.com/sumandas27/scratchOS-Blog).

At this point, all our work for the rest of this section will be within `stage_1/main.rs` only.

## Processor Registers

Just like how secondary storage and RAM were types of memory, *processor registers* are another type of memory. They are tiny pieces of storage that lives directly on CPU. Because they are so close to the CPU, operating with a register's memory is *by far* the fastest out of any memory type. However, registers are *extremely small* (8 bytes wide in general, which is the case on x86 machines as well), and there aren't many of them we can use. In fact, each register on the CPU has their own 2 to 3 letter name like `rax`, `rbx`, `rsp`, etc.

<p align="center">
  <img width="270px" src="img/cpu-register-diagram.png">
</p>

Recall how I mentioned that our emulator starts in *16-bit real mode* (while constructing the custom target JSON file). This means that at this current emulator state, we can only use a quarter of every register. A typical register is 8 bytes long, so a quarter of it is 2 bytes, the same as 16 bits, thus the name *16-bit real mode*.

<p align="center">
  <img src="img/register-anatomy.png">
</p>

## The Program Stack

## Memory Segmentation in 16-Bit Mode

## Assembly in Rust

## The Basic Input Output System

## A Bootloader `Hello, World!`