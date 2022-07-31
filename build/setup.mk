ifeq ($(MODE), release)
CARGOFLAGS+=--release
endif

setup:
	mkdir bin