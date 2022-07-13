#[cfg(target_arch = "avr")]
pub type Mutex<T> = crate::arch::avr::Mutex<T>;

#[cfg(not(target_arch = "avr"))]
pub type Mutex<T> = parking_lot::Mutex<T>;
