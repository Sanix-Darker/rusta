# RUSTA

**Unsafe** Rust runtime that boots bare-metal on Raspberry Pi 3 / 4 / 5—think *Arduino-style* simplicity, on a 64-bit quad-core.

[![Built with Rust](https://img.shields.io/badge/Built%20with-Rust-orange)](https://www.rust-lang.org)
[![License MIT](https://img.shields.io/badge/License-MIT-blue)](LICENSE)

Inspired by
* Low-Level TV – <https://www.youtube.com/watch?v=jZT8APrzvc4>
* The legendary <https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials>

## Why **RUSTA** instead of existing `rust-embedded` OSes?

| Aspect | *TockOS / Hubris / Others* | **RUSTA** |
|--------|----------------------------|-----------|
| **Foot-print** | 50 – 150 KB kernel + tasks | **\< 16 KB total** – one flat binary(on huge project) |
| **Boot latency** | µ-loader then kernel init | **GPU → `kernel8.img` → your `main()`** |
| **Complexity** | Scheduler, syscalls, async | **Single core-loop**; you choose the pattern |
| **GPIO API** | HAL crate per board | **One-line `GPIO::write(pin, hi)`** |
| **Learning curve** | RTOS concepts | Pure Rust + the Pi datasheet |
| **Customization** | Fork the kernel | Just hack your crate; no kernel at all |

RUSTA is ideal when you **want absolute control** but still crave Rust’s safety and ergonomics—no pre-emptive kernel, no MMU tricks, just *you* and the silicon.


## Why **RUSTA on a Raspberry Pi 4** versus a few popular micro-controller boards.


| Feature / Metric     | RUSTA (R-Pi 4 @ 1.5 GHz) | Arduino Uno (AVR) | ESP32-WROOM (2×240 MHz) |
|----------------------|-----------------------|--------------------|---------------------|
| CPU cores / ISA      | ► 4× Cortex-A72, 64-bit | 1× AVR, 8-bit     | 2× Xtensa LX6, 32-bit |
| Clock frequency      | ► 1 500 MHz           | 16 MHz             | 240 MHz             |
| RAM available        | ► 2-8 GB DDR4         | 2 KB SRAM          | 520 KB SRAM         |
| Program storage      | ► SD-card (GBs)       | 32 KB flash        | 4 MB flash          |
| Language safety      | ► Rust, zero-cost     | C / C++ manual     | C / C++ manual      |
| MMU / virt. mem      | ► Yes (bare-metal)    | No                 | No                  |
| Concurrency model    | ► SMP threads or DIY  | Single loop        | FreeRTOS tasks      |
| Peripheral count     | ► 40 GPIO + USB 3/2   | 14 GPIO + USB 2    | 34 GPIO + Wi-Fi/BT  |
| Console & logging    | ► `println!()` via UART | `Serial.print()` | `Serial.printf()`   |
| Typical toolchain    | ► `rustup + cargo`    | `avr-gcc`, avrdude | ESP-IDF (CMake)     |
| Build footprint      | ► \< 32 KB firmware   | 2–32 KB sketch     | 200–500 KB bin      |
| Boot latency         | ► \< 200 ms (GPU→code) | \< 50 ms           | ~200 ms             |
| Idle power draw†     | ~0.5 W (Pi Zero: 0.1 W)| ► ~50 mW          | ~20 mW              |

RUSTA trades raw performance for higher power; micro-controllers still win on ultra-low-power duty-cycling.

## Feature Highlights

- **Unified Board Support** – Pi 3, 4, 5 selectable via Cargo features.
- **Rich Peripheral Drivers** – GPIO, UART (with `println!`), PWM, SPI, I²C, delay cycles.
- **Zero Dynamic Allocation** – no `alloc`, no heap.
- **`unsafe` Where It Counts** – memory-mapped I/O wrapped once, so you use safe methods.
- **One-Command Build** – `make image` → `kernel8.img`.
- **Path / Git / crates.io Friendly** – drop it in as a dependency, no custom tool needed.
- **Serial Console** – `print!` / `println!` out of the box at 115 200 baud.

## Quick Blink

```rust
#![no_std]
#![no_main]
extern crate panic_halt;

use rusta::{delay, gpio::{GPIO, Mode}};

#[no_mangle]
fn _start() -> ! {
    const LED: usize = 21;        // activity LED on Pi-4
    GPIO::set_mode(LED, Mode::Output);

    loop {
        GPIO::write(LED, true);
        delay::cycles(50_000);    // ~5 s @ 19.2 MHz timer
        GPIO::write(LED, false);
        delay::cycles(50_000);
    }
}
```

Then build and generate the image with :

```bash
$ make build
cargo +nightly build  \
    --release --target aarch64-unknown-none \
    --example blink --features pi3
    Finished `release` profile [optimized] target(s) in 0.02s

$ make image
cargo +nightly build  \
    --release --target aarch64-unknown-none \
    --example blink --features pi3
    Finished `release` profile [optimized] target(s) in 0.01s
rust-objcopy -O binary target/aarch64-unknown-none/release/examples/blink kernel8.img

$ ls -alh ./kernel8.img
Permissions Size User Date Modified Name
.rwxrwxr-x  4.4k dk   27 May 18:56  ./kernel8.img
```

## Toolchain Setup

```bash
# one-time: nightly toolchain & aarch64 target
rustup toolchain install nightly
rustup component add rust-src           --toolchain nightly
rustup target    add aarch64-unknown-none --toolchain nightly
```

(or simply `make setup`)

---

## SD-Card Contents

```console
# essential boot files (Pi firmware)
wget https://github.com/raspberrypi/firmware/raw/master/boot/bootcode.bin
wget https://github.com/raspberrypi/firmware/raw/master/boot/start.elf
wget https://github.com/raspberrypi/firmware/raw/master/boot/fixup.dat

# your bare-metal binary
cp kernel8.img /media/$USER/boot/

# optional minimal config.txt
echo 'disable_fw_kill=1' > /media/$USER/boot/config.txt
```

Format the card as **FAT32**, copy the files, eject, power-cycle—LED blinks, serial prints, you win. 🚀

## Author

- [sanixdk](https://github.com/sanix-darker)
