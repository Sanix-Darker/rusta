use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll, RawWaker, RawWakerVTable, Waker},
};

pub struct Executor {
    tasks: [Option<Pin<Box<dyn Future<Output = ()>>>>; 32],
}

impl Executor {
    pub fn new() -> Self {
        Self { tasks: [None; 32] }
    }

    pub fn spawn(&mut self, fut: impl Future<Output = ()> + 'static) {
        for slot in &mut self.tasks {
            if slot.is_none() {
                *slot = Some(Box::pin(fut));
                return;
            }
        }
        panic!("Task queue full");
    }

    pub fn run(&mut self) -> ! {
        loop {
            for task in &mut self.tasks {
                if let Some(fut) = task {
                    let waker = dummy_waker();
                    let mut cx = Context::from_waker(&waker);

                    if let Poll::Ready(()) = fut.as_mut().poll(&mut cx) {
                        *task = None;
                    }
                }
            }
            // Sleep until next interrupt
            unsafe { asm!("wfi") };
        }
    }
}

fn dummy_waker() -> Waker {
    unsafe { Waker::from_raw(dummy_raw_waker()) }
}

fn dummy_raw_waker() -> RawWaker {
    RawWaker::new(0 as *const (), &VTABLE)
}

static VTABLE: RawWakerVTable = RawWakerVTable::new(
    |_| dummy_raw_waker(),
    |_| {},
    |_| {},
    |_| {},
);
