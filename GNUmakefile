QEMU_ARGS=-M q35 -m 2G -boot d -serial stdio -no-reboot  #-S -s
KERNEL_LIB=target/x86_64-moon_os/debug/libmoon_os.a
KERNEL=target/x86_64-moon_os/debug/moon_os
ISO=moon_os.iso
HDD=moon_os.hdd

.PHONY: all
all: $(ISO)

.PHONY: all-hdd
all-hdd: $(HDD)

.PHONY: run
run: $(ISO)
	qemu-system-x86_64 $(QEMU_ARGS) -cdrom $(ISO)

.PHONY: run-uefi
run-uefi: ovmf-x64 $(ISO)
	qemu-system-x86_64 $(QEMU_ARGS) -bios ovmf-x64/OVMF.fd -cdrom $(ISO)

.PHONY: run-hdd
run-hdd: $(HDD)
	qemu-system-x86_64 $(QEMU_ARGS) -hda $(HDD)

.PHONY: run-hdd-uefi
run-hdd-uefi: ovmf-x64 $(HDD)
	qemu-system-x86_64 $(QEMU_ARGS) -bios ovmf-x64/OVMF.fd -hda $(HDD)

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

$(ISO): limine kernel
	rm -rf iso_root
	mkdir -p iso_root
	cp $(KERNEL) \
		limine.cfg limine/limine.sys limine/limine-cd.bin limine/limine-cd-efi.bin iso_root/
	xorriso -as mkisofs -b limine-cd.bin \
		-no-emul-boot -boot-load-size 4 -boot-info-table \
		--efi-boot limine-cd-efi.bin \
		-efi-boot-part --efi-boot-image --protective-msdos-label \
		iso_root -o $(ISO)
	limine/limine-deploy $(ISO)
	rm -rf iso_root

$(HDD): limine kernel
	rm -f $(HDD)
	dd if=/dev/zero bs=1M count=0 seek=64 of=$(HDD)
	parted -s $(HDD) mklabel gpt
	parted -s $(HDD) mkpart ESP fat32 2048s 100%
	parted -s $(HDD) set 1 esp on
	limine/limine-deploy $(HDD)
	sudo losetup -Pf --show $(HDD) >loopback_dev
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
	rm -rf iso_root $(ISO) $(HDD) src/stack.o
	cargo clean

.PHONY: distclean
distclean: clean
	rm -rf limine ovmf-x64
