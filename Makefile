TARGET  := aarch64-unknown-none
FEATURE ?= pi3
EXAMPLE ?= blink
OBJCOPY := rust-objcopy
OBJDUMP := rust-objdump
TOOLCHAIN ?= nightly
BUILD_STD  := -Z build-std=core

# build specific example
build:
	cargo +$(TOOLCHAIN) build $(BUILDSTD) \
	    --release --target $(TARGET) \
	    --example $(EXAMPLE) --features $(FEATURE)

# build only the library (no examples)
lib:
	cargo +$(TOOLCHAIN) build $(BUILDSTD) \
	    --release --target $(TARGET) --features $(FEATURE)

setup:
	rustup toolchain install $(TOOLCHAIN)
	rustup component add rust-src --toolchain $(TOOLCHAIN)
	rustup target   add $(TARGET) --toolchain $(TOOLCHAIN)
	cargo +nightly install cargo-binutils
	sudo apt-get install binutils-aarch64-linux-gnu -y

# disassemble example ELF
dump: build
	$(OBJDUMP) -d target/$(TARGET)/release/examples/$(EXAMPLE) | less

image: build
	$(OBJCOPY) -O binary target/$(TARGET)/release/examples/$(EXAMPLE) kernel8.img

format:
	cargo +$(TOOLCHAIN) fmt --all

flash: image
	@echo "Copy kernel8.img to SD‑card boot partition and reboot the Pi"

clean:
	cargo clean

.PHONY: build image flash clean dump setup
