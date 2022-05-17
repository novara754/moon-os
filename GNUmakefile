QEMU_ARGS=-M q35 -m 2G -boot d -serial stdio -no-reboot  #-S -s
KERNEL_LIB=target/x86_64-moon_os/debug/libmoon_os.a
KERNEL=target/x86_64-moon_os/debug/moon_os

.PHONY: all
all: barebones.iso

.PHONY: all-hdd
all-hdd: barebones.hdd

.PHONY: run
run: barebones.iso
	qemu-system-x86_64 $(QEMU_ARGS) -cdrom barebones.iso

.PHONY: run-uefi
run-uefi: ovmf-x64 barebones.iso
	qemu-system-x86_64 $(QEMU_ARGS) -bios ovmf-x64/OVMF.fd -cdrom barebones.iso

.PHONY: run-hdd
run-hdd: barebones.hdd
	qemu-system-x86_64 $(QEMU_ARGS) -hda barebones.hdd

.PHONY: run-hdd-uefi
run-hdd-uefi: ovmf-x64 barebones.hdd
	qemu-system-x86_64 $(QEMU_ARGS) -bios ovmf-x64/OVMF.fd -hda barebones.hdd

ovmf-x64:
	mkdir -p ovmf-x64
	cd ovmf-x64 && curl -o OVMF-X64.zip https://efi.akeo.ie/OVMF/OVMF-X64.zip && 7z x OVMF-X64.zip

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

barebones.iso: limine kernel
	rm -rf iso_root
	mkdir -p iso_root
	cp $(KERNEL) \
		limine.cfg limine/limine.sys limine/limine-cd.bin limine/limine-cd-efi.bin iso_root/
	xorriso -as mkisofs -b limine-cd.bin \
		-no-emul-boot -boot-load-size 4 -boot-info-table \
		--efi-boot limine-cd-efi.bin \
		-efi-boot-part --efi-boot-image --protective-msdos-label \
		iso_root -o barebones.iso
	limine/limine-deploy barebones.iso
	rm -rf iso_root

barebones.hdd: limine kernel
	rm -f barebones.hdd
	dd if=/dev/zero bs=1M count=0 seek=64 of=barebones.hdd
	parted -s barebones.hdd mklabel gpt
	parted -s barebones.hdd mkpart ESP fat32 2048s 100%
	parted -s barebones.hdd set 1 esp on
	limine/limine-deploy barebones.hdd
	sudo losetup -Pf --show barebones.hdd >loopback_dev
	sudo mkfs.fat -F 32 `cat loopback_dev`p1
	mkdir -p img_mount
	sudo mount `cat loopback_dev`p1 img_mount
	sudo mkdir -p img_mount/EFI/BOOT
	sudo cp -v $(KERNEL) limine.cfg limine/limine.sys img_mount/
	sudo cp -v limine/BOOTX64.EFI img_mount/EFI/BOOT/
	sync
	sudo umount img_mount
	sudo losetup -d `cat loopback_dev`
	rm -rf loopback_dev img_mount

.PHONY: clean
clean:
	rm -rf iso_root barebones.iso barebones.hdd src/stack.o
	cargo clean

.PHONY: distclean
distclean: clean
	rm -rf limine ovmf-x64
