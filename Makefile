# Define default target and feature
TARGET_BARE_METAL := aarch64-unknown-none
FEATURE ?= pi3
EXAMPLE ?= blink
TOOLCHAIN ?= nightly
BUILD_STD := -Z build-std=core # Corrected variable name from BUILDSTD

# Linux specific targets
TARGET_LINUX_X86_64 := x86_64-unknown-linux-gnu
TARGET_LINUX_AARCH64 := aarch64-unknown-linux-gnu

# Binutils for bare-metal and cross-compilation
OBJCOPY := rust-objcopy
OBJDUMP := rust-objdump

# Default build target (bare-metal)
.PHONY: build
build:
	@echo "Building example '$(EXAMPLE)' for bare-metal target: $(TARGET_BARE_METAL) with feature: $(FEATURE)"
	cargo +$(TOOLCHAIN) build $(BUILD_STD) \
		--release --target $(TARGET_BARE_METAL) \
		--example $(EXAMPLE) --features $(FEATURE)

# Default library build target (bare-metal)
.PHONY: lib
lib:
	@echo "Building library for bare-metal target: $(TARGET_BARE_METAL) with feature: $(FEATURE)"
	cargo +$(TOOLCHAIN) build $(BUILD_STD) \
		--release --target $(TARGET_BARE_METAL) --features $(FEATURE)

# Build example for x86_64 Linux
.PHONY: build-linux-x86_64
build-linux-x86_64:
	@echo "Building example '$(EXAMPLE)' for x86_64 Linux target: $(TARGET_LINUX_X86_64) with feature: $(FEATURE)"
	cargo +$(TOOLCHAIN) build \
		--release --target $(TARGET_LINUX_X86_64) \
		--example $(EXAMPLE) --features $(FEATURE)

# Build library for x86_64 Linux
.PHONY: lib-linux-x86_64
lib-linux-x86_64:
	@echo "Building library for x86_64 Linux target: $(TARGET_LINUX_X86_64) with feature: $(FEATURE)"
	cargo +$(TOOLCHAIN) build \
		--release --target $(TARGET_LINUX_X86_64) --features $(FEATURE)

# Build example for aarch64 Linux
.PHONY: build-linux-aarch64
build-linux-aarch64:
	@echo "Building example '$(EXAMPLE)' for aarch64 Linux target: $(TARGET_LINUX_AARCH64) with feature: $(FEATURE)"
	cargo +$(TOOLCHAIN) build \
		--release --target $(TARGET_LINUX_AARCH64) \
		--example $(EXAMPLE) --features $(FEATURE)

# Build library for aarch64 Linux
.PHONY: lib-linux-aarch64
lib-linux-aarch64:
	@echo "Building library for aarch64 Linux target: $(TARGET_LINUX_AARCH64) with feature: $(FEATURE)"
	cargo +$(TOOLCHAIN) build \
		--release --target $(TARGET_LINUX_AARCH64) --features $(FEATURE)

# All Linux builds
.PHONY: all-linux-build
all-linux-build: build-linux-x86_64 build-linux-aarch64

.PHONY: all-linux-lib
all-linux-lib: lib-linux-x86_64 lib-linux-aarch64

.PHONY: all-linux
all-linux: all-linux-build all-linux-lib

# Setup Rust toolchains and components
.PHONY: setup
setup:
	@echo "Setting up Rust toolchain and targets..."
	rustup toolchain install $(TOOLCHAIN)
	rustup component add rust-src --toolchain $(TOOLCHAIN)
	rustup target add $(TARGET_BARE_METAL) --toolchain $(TOOLCHAIN)
	rustup target add $(TARGET_LINUX_X86_64) --toolchain $(TOOLCHAIN)
	rustup target add $(TARGET_LINUX_AARCH64) --toolchain $(TOOLCHAIN)
	cargo +nightly install cargo-binutils
	sudo apt-get update && sudo apt-get install binutils-aarch64-linux-gnu -y

# Disassemble example ELF (bare-metal)
.PHONY: dump
dump: build
	@echo "Disassembling bare-metal example ELF..."
	$(OBJDUMP) -d target/$(TARGET_BARE_METAL)/release/examples/$(EXAMPLE) | less

# Create kernel image (bare-metal)
.PHONY: image
image: build
	@echo "Creating kernel8.img from bare-metal example..."
	$(OBJCOPY) -O binary target/$(TARGET_BARE_METAL)/release/examples/$(EXAMPLE) kernel8.img

# Format Rust code
.PHONY: format
format:
	@echo "Formatting Rust code..."
	cargo +$(TOOLCHAIN) fmt --all

# Flash instructions for bare-metal
.PHONY: flash
flash: image
	@echo "Copy kernel8.img to SD-card boot partition and reboot the Pi"

# Clean build artifacts
.PHONY: clean
clean:
	@echo "Cleaning Cargo build artifacts..."
	cargo clean

