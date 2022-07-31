.ONESHELL:

.PHONY: kernel
kernel:
	cd kernel
	cargo build --profile $(MODE) --target ../build/targets/$(ARCH)/kernel.json $(CARGOFLAGS)
	cd ..
	cp target/kernel/$(FOLDER)/kernel bin/clecx
	strip bin/clecx



kernel/asm:
	cd kernel
	cargo rustc --target ../build/targets/$(ARCH)/kernel.json $(CARGOFLAGS) -- --emit asm
	cd ..
