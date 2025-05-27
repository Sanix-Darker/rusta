#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        let _ = core::fmt::write(&mut $crate::uart::UART::writer(),
                                 format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! println {
    () => { $crate::print!("
"); };
    ($fmt:expr) => { $crate::print!(concat!($fmt, "
")); };
    ($fmt:expr, $($arg:tt)*) => { $crate::print!(concat!($fmt, "
"), $($arg)*); };
}
