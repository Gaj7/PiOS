deafult: elf

.PHONY: build boot elf img run clean

#in the future, may need to add a "--cfg" for pi zero/1 vs 2/3, maybe an option for qemu too
build:
	xargo build --target=arm-none-eabihf

boot:
	arm-none-eabi-gcc -mcpu=arm1176jzf-s -fpic -ffreestanding -c boot.S -o boot.o

elf: build boot
	arm-none-eabi-gcc -Wl,--gc-sections -T linker.ld -o piOS.elf -ffreestanding -O2 -nostartfiles -nostdlib boot.o ./target/arm-none-eabihf/debug/libPiOS.a

img: elf
	qemu-img convert piOS.elf kernel.img

run: elf
	qemu-system-arm -kernel piOS.elf -nographic -machine raspi2

clean:
	rm -f boot.o piOS.elf kernel.img
	cargo clean
