pub mod pi3;
pub mod pi4;
pub mod pi5;

// Re‑export the active board’s constants (GPIO_BASE, etc.)
#[cfg(all(not(feature = "pi4"), not(feature = "pi5")))]
pub use pi3::*;
#[cfg(feature = "pi4")]
pub use pi4::*;
#[cfg(feature = "pi5")]
pub use pi5::*;
