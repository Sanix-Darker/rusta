use crate::i2c::I2C;

#[derive(Clone, Copy)]
pub enum Font {
    Small,
    Medium,
    Large,
}

pub enum Fill {
    Solid,
    Outline,
}

pub struct OLED {
    addr: u8,
    buffer: [u8; 1024], // 128x64 monochrome
}

impl OLED {
    pub fn new(addr: u8) -> Self {
        Self {
            addr,
            buffer: [0; 1024],
        }
    }

    pub fn init(&mut self) {
        I2C::write(self.addr, &[
            0x00, // Command stream
            0xAE, // Display off
            0xD5, 0x80, // Set display clock div
            0xA8, 0x3F, // Set multiplex
            0xD3, 0x00, // Set display offset
            0x40, // Set start line
            0x8D, 0x14, // Charge pump
            0x20, 0x00, // Memory mode
            0xA1, // Segment remap
            0xC8, // COM output scan direction
            0xDA, 0x12, // Set COM pins
            0x81, 0xCF, // Set contrast
            0xD9, 0xF1, // Set precharge
            0xDB, 0x40, // Set vcom detect
            0xA4, // Display resume
            0xA6, // Normal display
            0xAF, // Display on
        ]);
    }

    pub fn clear(&mut self) {
        self.buffer = [0; 1024];
    }

    pub fn pixel(&mut self, x: u8, y: u8, on: bool) {
        if x < 128 && y < 64 {
            let idx = (y as usize / 8) * 128 + x as usize;
            let bit = y % 8;
            if on {
                self.buffer[idx] |= 1 << bit;
            } else {
                self.buffer[idx] &= !(1 << bit);
            }
        }
    }

    pub fn flush(&self) {
        let mut cmds = [0x00, 0x21, 0, 127, 0x22, 0, 7];
        I2C::write(self.addr, &cmds);
        I2C::write(self.addr, &[0x40]); // Data stream
        I2C::write(self.addr, &self.buffer);
    }
}
