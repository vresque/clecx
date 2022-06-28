image:
	dd if=/dev/zero of=$(IMAGE) bs=512 count=93750
	mkfs.vfat -F32 $(IMAGE)
	mmd -i $(IMAGE) ::/EFI
	mmd -i $(IMAGE) ::/EFI/BOOT
	mcopy -i $(IMAGE) bin/BOOTX64.EFI ::/EFI/BOOT
	mcopy -i $(IMAGE) scripts/startup.nsh ::
	mcopy -i $(IMAGE) bin/clecx ::
	mcopy -i $(IMAGE) res/font.psf ::
	mcopy -i $(IMAGE) res/FONT_LICENSE ::

