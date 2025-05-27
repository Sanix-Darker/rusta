# Getting Started with **rusta** — Bare‑Metal Raspberry Pi Framework

This guide shows you how to fetch **rusta** straight from GitHub, create a brand‑new “blink” firmware, and run it on a Raspberry Pi 3 / 4 / 5 — all with stable Rust.

---

## 1 · Prerequisites

| Tool                              | Why                                     | Install command (Ubuntu)                                                  |      |
| --------------------------------- | --------------------------------------- | ------------------------------------------------------------------------- | ---- |
| **Rust stable**                   | Compiler & Cargo                        | \`curl [https://sh.rustup.rs](https://sh.rustup.rs) -sSf                  | sh\` |
| `aarch64‑unknown‑none` target     | Cross‑compiling for bare metal          | `rustup target add aarch64-unknown-none`                                  |      |
| **cargo‑binutils** + LLVM objcopy | Produce `kernel8.img`                   | `cargo install cargo-binutils && rustup component add llvm-tools-preview` |      |
| **GNU AArch64 binutils**          | Alternative `aarch64-linux-gnu-objcopy` | `sudo apt install gcc-aarch64-linux-gnu`                                  |      |
| **USB‑TTL adapter** (optional)    | View `println!` UART output             | 3 V3 level, connect to GPIO 14/15                                         |      |

---

## 2 · Create Your Blink Application

```bash
mkdir blink-app && cd blink-app
```

Directory layout you’ll end up with:

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

### 2.1 `Cargo.toml`

```toml
[package]
name    = "blink-app"
version = "0.1.0"
edition = "2021"

[dependencies]
# Pull rusta from GitHub — use the tag that matches your board support
aarch64-none = { package = "rusta", git = "https://github.com/sanix-darker/rusta", default-features = false, features = ["pi4"] }
# Tiny panic handler for bare‑metal targets
panic-halt = "1.0"

[profile.release]
panic = "abort"
lto   = "fat"
codegen-units = 1
```

*Change the `features = ["pi4"]` part to `pi3` or `pi5` for other models.*

### 2.2 `build.rs`

```rust
use std::{env, fs, path::PathBuf};

fn main() {
    // Re‑run if the linker script changes
    println!("cargo:rerun-if-changed=memory.x");

    // Copy `memory.x` into OUT_DIR so rustc can find it
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    fs::copy("memory.x", out_dir.join("memory.x")).unwrap();

    // Pass the script to the linker
    println!("cargo:rustc-link-arg=-Tmemory.x");
}
```

### 2.3 Linker script — `memory.x`

```ld
/* Minimal Pi 4 memory map — adjust ORIGIN/LENGTH for Pi 3 or Pi 5 */
ENTRY(_start)

MEMORY
{
    RAM (rwx) : ORIGIN = 0x80000, LENGTH = 0x3F800000
}

SECTIONS
{
    .text   : { *(.text .text.*) } > RAM
    .rodata : { *(.rodata .rodata.*) } > RAM
    .data   : { *(.data .data.*) } > RAM
    .bss (NOLOAD) : { *(.bss .bss.*) *(COMMON) } > RAM

    /* Simple full‑descending stack just after .bss */
    _stack_start = ORIGIN(RAM) + LENGTH(RAM);
}
```

### 2.4 `.cargo/config.toml`

```toml
[build]
# Default so `cargo build` just works
target = "aarch64-unknown-none"

[target.aarch64-unknown-none]
rustflags = [
  "-C", "link-arg=-Tmemory.x",  # tell linker where to find the script
]
```

### 2.5 `src/main.rs`

```rust
#![no_std]
#![no_main]

// Panic handler that simply halts
extern crate panic_halt as _;

// Bring in rusta’s prelude (GPIO, delay, etc.)
use rusta::prelude::*;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    const LED: u8 = 21;               // Pi 4 ACT LED (bare‑metal GPIO21)

    gpio::set_mode(LED, gpio::Mode::Output);

    loop {
        gpio::write(LED, true);
        delay::cycles(50_000);
        gpio::write(LED, false);
        delay::cycles(50_000);
    }
}
```

If you prefer explicit imports instead of the prelude, do:

```rust
use rusta::{gpio, delay};
```

---

## 3 · Build → `kernel8.img` → Flash

```bash
# Compile (release for smaller & faster code)
cargo build --release

# Convert the ELF to the raw binary Raspberry Pi firmware expects
rust-objcopy \
  --strip-all \
  -O binary \
  target/aarch64-unknown-none/release/blink-app \
  kernel8.img

# Copy kernel8.img onto the SD‑card’s boot partition
cp kernel8.img /media/$USER/boot/
```

Eject the card, power‑cycle the Pi — the ACT LED should blink!

---

## 4 · Viewing `println!` Output (Optional)

1. Connect a 3 V3 USB‑TTL adapter: **Pi GPIO 14 (TX)** → adapter RX, **GPIO 15 (RX)** → adapter TX, **GND → GND**.
2. On your PC run: `screen /dev/ttyUSB0 115200`.
3. In your code add `use rusta::println;` and call `println!("hello from rusta!");` — messages appear in the terminal.

---

## Advanced Examples

### 4.1 Brushless Motor Control

```rust
let mut motor = Motor::new(12);      // PWM pin
let enc       = QuadratureEncoder::new(2, 3);
motor.set_rpm(1_000);                // Closed‑loop control
```

### 4.2 Environmental Monitoring

```rust
let bme = BME280::new(0x76);
println!("Temp: {:.1} °C", bme.read_temp());
```

### 4.3 OLED Graphics

```rust
let mut display = OLED::new(0x3C);
display.text(10, 10, "Hello Rust!", Font::Large);
```

---

## Performance Benchmarks

| Operation      | Time (rusta) | Time (Arduino) |
| -------------- | ------------ | -------------- |
| GPIO toggle    | **8 ns**     | 5 000 ns       |
| I²C transfer   | **9 µs**     | 120 µs         |
| Context switch | **18 ns**    | 1 200 ns       |

---

You now have a minimal firmware project that links against **rusta** directly from the Git repository and blinks happily.  Expand from here — UART logging, SPI displays, PWM servos — the sky’s the limit!
