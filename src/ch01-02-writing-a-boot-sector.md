# Writing A Boot Sector

Now that we have a freestanding Rust binary designed for the x86 architecture, let's run it on an x86 machine...

## The QEMU Emulator

Testing your bootloader and your operating system on an actual computer is difficult as every time any change is made, you need to burn your project onto a USB stick and reboot your machine. If you develop with only one machine, constantly switching and rebooting between the operating system your making and the operating system you're developing on will really slow down any progress you're making. 

Instead, we are going to use an *emulator*, which emulates the hardware of a regular computer. This allows you to develop and test on one machine without needing constant reboots.

`QEMU` is definitely the most popular emulator used for OS development. You can find installation instructions at <https://www.qemu.org/download/>. Usually, it's just one terminal command.

Installing `QEMU` should allow you to use the `qemu-system-x86_64` command, which emulates an x86 machine. Run `qemu-system-x86_64 --version` on your terminal to verify that it's there. This is nice because after we are done with setup, it won't matter which architecture your machine uses as we will all be developing for x86 through the `QEMU` emulator.