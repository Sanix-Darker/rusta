/*======================================================================
  Minimal VideoCore mailbox / framebuffer interface
  ======================================================================*/

use crate::{board::PERIPHERAL_BASE, volatile::Volatile};

const MB_BASE:        usize = PERIPHERAL_BASE + 0x00B880;
const MB_READ:        usize = MB_BASE + 0x00;
const MB_STATUS:      usize = MB_BASE + 0x18;
const MB_WRITE:       usize = MB_BASE + 0x20;
const STATUS_FULL:    u32   = 1 << 31;
const STATUS_EMPTY:   u32   = 1 << 30;
const CH_PROP:        u32   = 8;          // property-tag channel

#[repr(C, align(16))]
struct MsgBuf {
    size:  u32,
    code:  u32,
    data:  [u32; 256],
}

pub struct Mailbox {
    buf: &'static mut MsgBuf,
}

#[derive(Debug)]
pub struct FrameBuffer {
    pub addr:  u32,
    pub size:  u32,
    pub pitch: u32,
    pub w:     u32,
    pub h:     u32,
    pub depth: u32,
}

impl Mailbox {
    /// Create mailbox using static buffer @ 0x1000 (L2-uncached).
    pub fn new() -> Self {
        unsafe {
            let buf = &mut *(0x1000 as *mut MsgBuf);
            Self { buf }
        }
    }

    fn call(&mut self, chan: u32) -> bool {
        unsafe {
            while Volatile::<u32>::read(&(MB_STATUS as *const u32)) & STATUS_FULL != 0 {}
            Volatile::<u32>::write(&(MB_WRITE as *const u32 as *mut u32),
                                    (self.buf as *mut _ as u32 & !0xF) | chan);

            loop {
                while Volatile::<u32>::read(&(MB_STATUS as *const u32)) & STATUS_EMPTY != 0 {}
                let resp = Volatile::<u32>::read(&(MB_READ as *const u32));
                if (resp & 0xF) == chan &&
                   (resp & !0xF) == (self.buf as *mut _ as u32) {
                    return self.buf.code == 0x8000_0000;
                }
            }
        }
    }

    /*------------------------------------------------------------------
      Property helpers
      ----------------------------------------------------------------*/
    /// Get firmware revision.
    pub fn firmware_rev(&mut self) -> Option<u32> {
        self.buf.data[..5].copy_from_slice(&[0x0000_0004, 0, 0, 0, 0]);
        self.buf.size = 5*4; self.buf.code = 0;

        if self.call(CH_PROP) { Some(self.buf.data[3]) } else { None }
    }

    /// Allocate framebuffer; returns descriptor on success.
    pub fn alloc_fb(&mut self, w: u32, h: u32, depth: u32) -> Option<FrameBuffer> {
        let mut idx = 0;
        macro_rules! tag {
            ($id:expr, $a:expr, $b:expr) => {{
                self.buf.data[idx  ] = $id;
                self.buf.data[idx+1] = 8;
                self.buf.data[idx+2] = 8;
                self.buf.data[idx+3] = $a;
                self.buf.data[idx+4] = $b;
                idx += 5;
            }};
        }
        tag!(0x00048003, w, h);       // phys size
        tag!(0x00048004, w, h);       // virt size
        tag!(0x00048005, depth, 0);   // bit depth
        self.buf.data[idx] = 0;       // end
        self.buf.size = (idx + 1) as u32 * 4;
        self.buf.code = 0;

        if !self.call(CH_PROP) { return None; }

        // Parse response for pitch + address
        let mut addr = 0; let mut size = 0; let mut pitch = 0;
        let mut off = 0;
        while self.buf.data[off] != 0 {
            let tag = self.buf.data[off];
            if tag == 0x00040001 { addr  = self.buf.data[off+3];
                                   size  = self.buf.data[off+4]; }
            if tag == 0x00040008 { pitch = self.buf.data[off+3]; }
            off += (self.buf.data[off+1]/4 + 3) as usize;
        }

        Some(FrameBuffer { addr, size, pitch, w, h, depth })
    }
}
