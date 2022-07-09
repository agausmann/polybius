use core::sync::atomic::{AtomicBool, Ordering};

use avr_hal_generic::avr_device::interrupt;
use lock_api::{GuardSend, RawMutex};

pub struct AvrMutex {
    locked: AtomicBool,
    reenable: AtomicBool,
}

unsafe impl RawMutex for AvrMutex {
    type GuardMarker = GuardSend;

    const INIT: Self = Self {
        locked: AtomicBool::new(false),
        reenable: AtomicBool::new(false),
    };

    fn lock(&self) {
        while !self.try_lock() {}
    }

    fn try_lock(&self) -> bool {
        let reenable = interrupt::disable();
        if self.locked.load(Ordering::SeqCst) {
            if reenable {
                unsafe { interrupt::enable() };
            }
            false
        } else {
            self.locked.store(true, Ordering::SeqCst);
            self.reenable.store(reenable, Ordering::SeqCst);
            true
        }
    }

    unsafe fn unlock(&self) {
        if self.locked.load(Ordering::SeqCst) {
            self.locked.store(false, Ordering::SeqCst);
            if self.reenable.load(Ordering::SeqCst) {
                unsafe { interrupt::enable() }
            }
        }
    }
}
