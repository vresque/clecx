.ONESHELL:

.PHONY: boot
boot:
	cd boot
	cargo build --target ../build/targets/$(ARCH)/boot.json $(CARGOFLAGS)
	cd ..
	cp target/boot/$(MODE)/boot.efi bin/BOOTX64.EFI
	strip bin/BOOTX64.EFI