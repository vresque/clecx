QEMUFLAGS += \
	-drive file=$(IMAGE),format=raw \
	-m 512M \
	-enable-kvm \
	-cpu host \
	-machine q35,accel=kvm:tcg \
	-drive if=pflash,format=raw,unit=0,file=res/OVMF/OVMF_CODE.fd,readonly=on \
	-drive if=pflash,format=raw,unit=1,file=res/OVMF/OVMF_VARS.fd \
	-d int,cpu_reset \
	-no-shutdown \
	-no-reboot \
	-smp 8 \
	-net none \
	-device isa-debug-exit,iobase=0xf4,iosize=0x04  \
	-vga std


run/qemu:
	qemu-system-$(ARCH) $(QEMUFLAGS)