deafult: elf

.PHONY: build boot elf img run clean

#in the future, may need to add a "--cfg" for pi zero/1 vs 2/3, maybe an option for qemu too
build:
	xargo build --target=arm-none-eabihf
	
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
	cargo clean
