use crate::{dma::DMA, pwm::PWM, volatile::Volatile};

pub struct Audio;

impl Audio {
    pub fn init(sample_rate: u32) {
        PWM::init(sample_rate, 0.5);
    }

    pub fn play(buffer: &[u16]) {
        unsafe {
            DMA::start_transfer(
                buffer.as_ptr() as *const u8,
                PWM::get_data_register() as *mut u8,
                buffer.len() * 2,
            );
        }
    }

    pub fn stop() {
        DMA::stop();
    }
}
