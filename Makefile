all: elf

build:
	xargo build --target arm-none-eabihf

boot:
	arm-none-eabi-gcc -mcpu=arm1176jzf-s -fpic -ffreestanding -c boot.S -o boot.o

elf: build boot
	arm-none-eabi-gcc -T linker.ld -o piOS.elf -ffreestanding -O2 -nostdlib boot.o ./target/arm-none-eabihf/debug/libkernel.rlib

img: elf
	arm-none-eabi-objcopy piOS.elf -O binary kernel.img

run: elf
	qemu-system-arm -M raspi2 -kernel piOS.elf -serial stdio

clean:
	rm -f boot.o piOS.elf kernel.img
