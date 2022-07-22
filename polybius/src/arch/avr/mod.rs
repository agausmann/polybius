#[cfg(feature = "atmega32u4")]
pub mod atmega32u4;

pub mod mutex;

pub use self::mutex::Mutex;
