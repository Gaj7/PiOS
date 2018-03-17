# PiOS: A small OS project for the Raspberry Pi
PiOS is a toy OS that I am developing for Raspberry Pi systems in order to learn more about operating systems, embedded development, and Rust, the language I am using for this project. Of course, this is a tiny OS, and its only purpose is as a pedagogical exercise. Some features I hope to implement include processes, virtual memory, and context switching.

## Setup
Make sure Rust Nightly is installed. We need to use the Nightly version of Rust because we will need to use a high number of unsafe features.

Then install Xargo. Xargo works just like Cargo, but makes cross-compiling much easier, as well as building without the standard library.

The last thing we need for compiling is to install the GCC toolchain for our target architecture, 'arm-none-eabi-gcc' (it may be called 'gcc-arm-none-eabi' when you are downloading it). We won't be using this for compiling, but we will need it for a little bit of assembling, and then linking everything together.

If you want to emulate the OS rather than testing on an actual Raspberry Pi, you are going to want to download qemu-system-arm, which is the the version of the QEMU emulator which, as the name suggests emulates, ARM systems.

## Compiling and Running
This project uses a Makefile rather than relying entirely on the Cargo build system because the compilation process is complicated by targeting a bare-metal system.
To build the project and run it in QEMU, use the command:
```
make run
```

To build this project for use on an actual Raspberry Pi, type:
```
make img
```
This builds the kernel and converts it to an image. You then will need to copy this kernel.img file to the SD card you wish to boot the Raspberry Pi from. Make sure the SD card also contains bootcode.bin and start.elf, provided by the people behind the Raspberry Pi, which basically acts as the firmware, and will execute our kernel image.

## Acknowledgments
Big shoutout to the OSdev wiki! They have a great introduction to getting a bare-metal project to boot on a Raspberry Pi, as well as setting up cross-compilers.

https://wiki.osdev.org/Raspberry_Pi_Bare_Bones

https://wiki.osdev.org/Raspberry_Pi_Bare_Bones_Rust
