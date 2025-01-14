.PHONY: clean all

all:
	cargo build --profile=stage-1 -Zbuild-std=core --target ./i386-code16-boot-sector.json -Zbuild-std-features=compiler-builtins-mem
	objcopy -I elf32-i386 -O binary ./target/i386-code16-boot-sector/stage-1/rust_boot ./disk_image.img
	nasm -f bin t.s
	cat t >> disk_image.img

clean:
	rm disk_image.img
