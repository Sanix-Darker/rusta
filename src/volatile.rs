use core::{marker::PhantomData, ptr};
#[repr(transparent)]
pub struct Volatile<T> {
    value: T,
    _pd: PhantomData<*mut T>,
}
impl<T> Volatile<T> {
    pub const fn new(value: T) -> Self {
        Self {
            value,
            _pd: PhantomData,
        }
    }
    #[inline(always)]
    pub fn read(&self) -> T
    where
        T: Copy,
    {
        unsafe { ptr::read_volatile(&self.value) }
    }
    #[inline(always)]
    pub fn write(&mut self, v: T) {
        unsafe { ptr::write_volatile(&mut self.value, v) }
    }
}
