use core::sync::atomic::{AtomicPtr, Ordering};

type Handler = fn();

static HANDLERS: [AtomicPtr<()>; 256] = {
    const INIT: AtomicPtr<()> = AtomicPtr::new(core::ptr::null_mut());
    [INIT; 256]
};

pub struct InterruptController;

impl InterruptController {
    pub fn new() -> Self {
        Self
    }

    pub fn register(&self, irq: u8, handler: Handler) {
        let handler_ptr = handler as *const () as *mut ();
        HANDLERS[irq as usize].store(handler_ptr, Ordering::SeqCst);
    }

    pub unsafe fn handle(&self, irq: u8) {
        let handler_ptr = HANDLERS[irq as usize].load(Ordering::SeqCst);
        if !handler_ptr.is_null() {
            let handler: Handler = core::mem::transmute(handler_ptr);
            handler();
        }
    }
}
