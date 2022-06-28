.ONESHELL:

.PHONY: kernel
kernel:
	cd kernel
	cargo build --target ../build/targets/$(ARCH)/kernel.json $(CARGOFLAGS)
	cd ..
	cp target/kernel/$(MODE)/kernel bin/clecx
	strip bin/clecx