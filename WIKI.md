# Getting Started with **rusta** — Bare‑Metal Raspberry Pi Framework

This short guide shows you how to fetch **rusta** from a Git repository, create a brand‑new application that blinks the on‑board LED, and flash it to a Raspberry Pi 3/4/5.

---

## 1 · Prerequisites

| Tool                           | Why                       | Command to install                                       |      |
| ------------------------------ | ------------------------- | -------------------------------------------------------- | ---- |
| **Rust** stable                | Compiler & Cargo          | \`curl [https://sh.rustup.rs](https://sh.rustup.rs) -sSf | sh\` |
| *aarch64‑unknown‑none* target  | Cross‑compiling           | `rustup target add aarch64-unknown-none`                 |      |
| **GNU AArch64 binutils**       | `objcopy` → `kernel8.img` | Ubuntu: `sudo apt install gcc-aarch64-linux-gnu`         |      |
| **USB‑TTL adapter** (optional) | See `println!` output     | 3 V3 level, connect to GPIO 14/15                        |      |

---

## 2 · Create Your App Crate

```bash
mkdir blink-app && cd blink-app
```

```
blink-app/
├── Cargo.toml
├── build.rs
├── memory.x
├── .cargo/
│   └── config.toml
└── src/
    └── main.rs
```

### 2.1 Cargo.toml

```toml
[package]
name    = "blink-app"
version = "0.1.0"
edition = "2021"

[dependencies]
# Pull rusta straight from GitHub:
rusta = { git = "https://github.com/sanix-darker/rusta", tag = "v0.0.1", features = ["pi4"] }
```

### 2.2 build.rs

```rust
use std::{env, fs, path::PathBuf};
fn main() {
    println!("cargo:rerun-if-changed=memory.x");
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    fs::copy("memory.x", out.join("memory.x")).unwrap();
    println!("cargo:rustc-link-arg=-Tmemory.x");
}
```

### 2.3 Linker script (memory.x)

```ld
ENTRY(_start)
MEMORY { RAM (rwx): ORIGIN = 0x80000, LENGTH = 0x800000 }
SECTIONS {
  .text   : { *(.text*)   } > RAM
  .rodata : { *(.rodata*) } > RAM
  .data   : { *(.data*)   } > RAM
  .bss (NOLOAD) : { *(.bss*) *(COMMON) } > RAM
  _stack_start = ORIGIN(RAM) + LENGTH(RAM);
}
```

### 2.4 .cargo/config.toml

```toml
[build]
target = "aarch64-unknown-none"

[target.aarch64-unknown-none]
rustflags = ["-C", "link-arg=-Tmemory.x"]
```

### 2.5 src/main.rs

```rust
#![no_std]
#![no_main]
extern crate panic_halt;

use rusta::{gpio::{GPIO, Mode}, delay};

#[no_mangle]
fn _start() -> ! {
    const LED: usize = 21; // Pi 4 activity LED (bare‑metal)

    GPIO::set_mode(LED, Mode::Output);

    loop {
        GPIO::write(LED, true);
        delay::cycles(50_000);
        GPIO::write(LED, false);
        delay::cycles(50_000);
    }
}
```

---

## 3 · Build, Package, Flash

```bash
# from blink-app/
cargo build --release            # cross‑compiles to ELF

# produce Raspberry Pi boot image
# or with rust-objcopy
aarch64-linux-gnu-objcopy -O binary \
    target/aarch64-unknown-none/release/blink-app \
    kernel8.img

# copy kernel8.img to the SD‑card’s boot partition
# (e.g. /media/$USER/boot) and safely eject
```

Power‑cycle the Pi: the ACT LED should blink.

---

## 4 · Viewing `println!` Output (Optional)

1. Connect a 3 V3 USB‑TTL adapter: Pi GPIO 14 (TX) → RX, GPIO 15 (RX) → TX, GND → GND.
2. Open a terminal on your PC: `screen /dev/ttyUSB0 115200`.
3. Add `use rusta::println;` in your code and call `println!("hello")` — text appears in the terminal.

---

You now have a minimal Pi firmware that links against **rusta** from Git and blinks happily. Extend from here — UART logging, SPI displays, PWM servos, the sky’s the limit!
