# RUSTA

**Unsafe** Rust “OS-like” runtime that boots bare-metal on Raspberry Pi 3 / 4 / 5, Arduino-style.

Inspired by
* Low Level’s video – <https://www.youtube.com/watch?v=jZT8APrzvc4>
* The excellent tutorial series – <https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials>

## Blink example

```rust
#![no_std]
#![no_main]

extern crate panic_halt;

use rusta::{
    delay,
    gpio::{Mode, GPIO},
};

#[no_mangle]
fn _start() -> ! {
    const LED: usize = 21;                // activity LED on Pi-4
    GPIO::set_mode(LED, Mode::Output);

    loop {
        GPIO::write(LED, true);
        delay::cycles(50_000);            // wait ~5 s

        GPIO::write(LED, false);
        delay::cycles(50_000);
    }
}
```

## Troubleshooting

```bash
# one-time toolchain prep
rustup toolchain install nightly
rustup component add rust-src           --toolchain nightly   # lets Cargo rebuild core
rustup target    add aarch64-unknown-none --toolchain nightly

# or in one go
make setup
```

### Required files on the SD card

```console
wget https://github.com/raspberrypi/firmware/raw/master/boot/bootcode.bin
wget https://github.com/raspberrypi/firmware/raw/master/boot/start.elf
wget https://github.com/raspberrypi/firmware/raw/master/boot/fixup.dat
# add your kernel8.img and (optionally) a minimal config.txt
# SD card must be FAT32
```
