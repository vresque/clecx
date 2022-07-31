.ONESHELL:

.PHONY: boot
boot:
	cd boot
	cargo build --profile $(MODE) --target ../build/targets/$(ARCH)/boot.json $(CARGOFLAGS)
	cd ..
	cp target/boot/$(FOLDER)/boot.efi bin/BOOTX64.EFI
	strip bin/BOOTX64.EFI