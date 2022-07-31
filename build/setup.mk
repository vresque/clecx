ifeq ($(MODE), release)
CARGOFLAGS+=--release
FOLDER=release
else
FOLDER=debug
endif

setup:
	mkdir bin