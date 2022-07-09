#[cfg(any(target_arch = "avr", test))]
pub type Mutex<T> = lock_api::Mutex<crate::arch::avr::mutex::AvrMutex, T>;
