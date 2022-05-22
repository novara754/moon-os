QEMU_ARGS=-M q35 -m 2G -boot d -serial stdio -no-reboot
KERNEL_LIB=target/x86_64-moon_os/debug/libmoon_os.a
KERNEL=target/x86_64-moon_os/debug/moon_os
TERMINAL_FONT=./terminal-font.psf
ISO=moon_os.iso

.PHONY: all
all: $(ISO)

.PHONY: all-hdd
all-hdd: $(HDD)

.PHONY: run
run: $(ISO)
	qemu-system-x86_64 $(QEMU_ARGS) -cdrom $(ISO)

.PHONY: run-debug
run-debug: $(ISO)
	qemu-system-x86_64 $(QEMU_ARGS) -S -s -cdrom $(ISO)

limine:
	git clone https://github.com/limine-bootloader/limine.git --branch=v3.0-branch-binary --depth=1
	make -C limine

.PHONY: kernel
kernel:
	cargo build
	nasm -f elf64 -o src/stack.o src/stack.s
	ld \
		-Tlinker.ld \
		-nostdlib \
		-zmax-page-size=0x1000 \
		-static \
		-o $(KERNEL) \
		$(KERNEL_LIB) \
		src/stack.o

$(ISO): limine kernel $(TERMINAL_FONT)
	rm -rf iso_root
	mkdir -p iso_root
	cp $(KERNEL) \
		$(TERMINAL_FONT) \
		limine.cfg limine/limine.sys limine/limine-cd.bin limine/limine-cd-efi.bin iso_root/
	xorriso -as mkisofs -b limine-cd.bin \
		-no-emul-boot -boot-load-size 4 -boot-info-table \
		--efi-boot limine-cd-efi.bin \
		-efi-boot-part --efi-boot-image --protective-msdos-label \
		iso_root -o $(ISO)
	limine/limine-deploy $(ISO)
	rm -rf iso_root

.PHONY: clean
clean:
	rm -rf iso_root $(ISO) $(HDD) src/stack.o
	cargo clean

.PHONY: distclean
distclean: clean
	rm -rf limine ovmf-x64
