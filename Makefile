.PHONY: clean all

all:
	cargo build -Zbuild-std=core --target ./i386-code16-boot-sector.json -Zbuild-std-features=compiler-builtins-mem
	objcopy -I elf32-i386 -O binary ./target/i386-code16-boot-sector/debug/rust_boot ./disk_image.img

clean:
	rm disk_image.img
