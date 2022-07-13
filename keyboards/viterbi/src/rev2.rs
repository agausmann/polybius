use atmega_hal::{
    clock::MHz16,
    delay::Delay,
    pac::{Peripherals, PLL, TC1, USB_DEVICE},
    port::{
        mode::{Floating, Output},
        PB1, PB2, PB3, PB4, PB5, PD1, PD4, PD7, PE6, PF4, PF5, PF6, PF7,
    },
    port::{
        mode::{Input, OpenDrain, PullUp},
        PB6,
    },
    port::{Pin, PD0},
};
use atmega_usbd::UsbBus;
use core::cmp::min;
use embedded_hal::blocking::delay::DelayUs;
use polybius::{
    diodes::ColToRow,
    keyboard::Keyboard,
    scanner::{Direct, ScanMatrix},
    uplink::usb::UsbHid,
};
use usb_device::{
    bus::UsbBusAllocator,
    device::{UsbDeviceBuilder, UsbVidPid},
};

pub const ROWS: usize = 10;
pub const COLS: usize = 7;

pub type ClockSpeed = MHz16;

pub type WriteLines = Direct<(
    Pin<OpenDrain, PD4>,
    Pin<OpenDrain, PD7>,
    Pin<OpenDrain, PE6>,
    Pin<OpenDrain, PB4>,
    Pin<OpenDrain, PB5>,
)>;
pub type ReadLines = Direct<(
    Pin<Input<PullUp>, PF4>,
    Pin<Input<PullUp>, PF5>,
    Pin<Input<PullUp>, PF6>,
    Pin<Input<PullUp>, PF7>,
    Pin<Input<PullUp>, PB1>,
    Pin<Input<PullUp>, PB3>,
    Pin<Input<PullUp>, PB2>,
)>;
pub type ScanDelay = fn();
pub type Diodes = ColToRow;
pub type Scanner = ScanMatrix<WriteLines, ReadLines, ScanDelay, Diodes, ROWS, COLS>;

fn scan_delay() {
    let mut delay = Delay::<ClockSpeed>::new();
    delay.delay_us(30_u8);
}

pub type Uplink = UsbHid<'static, UsbBus>;

pub struct Backlight {
    // 10.3.1: The PB6 pin can serve as an external output for the
    // Timer/Counter1 Output Compare B.
    // The pin has to be configured as an output to serve this function.
    pin: Pin<Output, PB6>,
    timer: TC1,
    current_level: u8,
}

impl Backlight {
    fn new(pb6: Pin<Input<Floating>, PB6>, tc1: TC1) -> Backlight {
        //TODO port TC0 -> TC1
        //
        // tc1.tccr0b.reset();
        // tc1.tccr0a.reset();
        // tc1.ocr0a.reset();
        // //NB: Output compare starts disconnected because initial level is zero.
        // // Setting OCR0A := 0 is not enough to turn off backlight;
        // // the pin must be fully disconnected.
        // tc1.tccr0a
        //     .write(|w| w.wgm0().pwm_fast().com0a().disconnected());
        // tc1.tccr0b.write(|w| w.cs0().direct().wgm02().clear_bit());

        Self {
            pin: pb6.into_output(),
            timer: tc1,
            current_level: 0,
        }
    }
}

//FIXME maybe non-linear relation between PWM duty and perceived brihtness?
const BACKLIGHT_LEVELS: u8 = 4;
// With 4 levels, the backlight value is contained in 2 bits.g
// To make it span an 8-bit value, it can be
// broadcast to each of the four 2-bit chunks:
const BACKLIGHT_OCR_MULT: u8 = 0b01010101;

impl polybius::backlight::Backlight for Backlight {
    fn num_levels(&self) -> u8 {
        BACKLIGHT_LEVELS
    }

    fn level(&self) -> u8 {
        self.current_level
    }

    fn set_level(&mut self, level: u8) {
        // TODO port TC0 -> TC1
        let level = min(level, self.num_levels() - 1);
        if level == 0 {
            // self.timer.tccr0a.modify(|_, w| w.com0a().disconnected());
            // self.pin.set_low();
        } else {
            // self.timer
            //     .ocr0a
            //     .write(|w| unsafe { w.bits(BACKLIGHT_OCR_MULT * level) });
            // self.timer.tccr0a.modify(|_, w| w.com0a().match_clear());
        }
        self.current_level = level;
    }
}

pub struct ViterbiRev2 {
    scanner: Scanner,
    uplink: Uplink,
    backlight: Backlight,
}

impl Keyboard<ROWS, COLS> for ViterbiRev2 {
    type Scanner = Scanner;

    type Uplink = Uplink;

    type Backlight = Backlight;

    fn scanner(&mut self) -> &mut Self::Scanner {
        &mut self.scanner
    }

    fn uplink(&mut self) -> &mut Self::Uplink {
        &mut self.uplink
    }

    fn backlight(&mut self) -> &mut Self::Backlight {
        &mut self.backlight
    }
}

impl ViterbiRev2 {
    /// Initialize the keyboard, taking full ownership of the device
    /// peripherals.
    ///
    /// # Panics
    ///
    /// This function calls `atmega_hal::Peripherals::take()` and will panic if
    /// the device peripherals have already been taken.
    ///
    /// If you want to keep ownership of the unused parts of the peripherals,
    /// use [`ViterbiRev2::from_parts`] or the [`from_parts!`] macro instead.
    pub fn new() -> Self {
        let dp = Peripherals::take().unwrap();
        let pins = atmega_hal::pins!(dp);
        from_parts!(dp, pins)
    }

    /// Initialize the keyboard, taking ownership of only the peripherals
    /// necessary.
    ///
    /// The [`from_parts!`] macro is a more convenient way to call this method.
    pub fn from_parts(
        pll: PLL,
        tc1: TC1,
        usb_device: USB_DEVICE,
        pb1: Pin<Input<Floating>, PB1>,
        pb2: Pin<Input<Floating>, PB2>,
        pb3: Pin<Input<Floating>, PB3>,
        pb4: Pin<Input<Floating>, PB4>,
        pb5: Pin<Input<Floating>, PB5>,
        pb6: Pin<Input<Floating>, PB6>,
        pd0: Pin<Input<Floating>, PD0>,
        pd1: Pin<Input<Floating>, PD1>,
        pd4: Pin<Input<Floating>, PD4>,
        pd7: Pin<Input<Floating>, PD7>,
        pe6: Pin<Input<Floating>, PE6>,
        pf4: Pin<Input<Floating>, PF4>,
        pf5: Pin<Input<Floating>, PF5>,
        pf6: Pin<Input<Floating>, PF6>,
        pf7: Pin<Input<Floating>, PF7>,
    ) -> Self {
        // Disable JTAG functionality to gain control of pins PF4-PF7.
        // This procedure has tight timing requirements (4 cycles between writes)
        // which can't be guaranteed by the codegen/linker with the safe code:
        // dp.JTAG.mcucr.modify(|_, w| w.jtd().set_bit());
        // dp.JTAG.mcucr.modify(|_, w| w.jtd().set_bit());
        #[cfg(target_arch = "avr")]
        unsafe {
            core::arch::asm!(
                "in r25, 0x35",
                "ori r25, 0x80",
                "out 0x35, r25",
                "out 0x35, r25",
                out("r25") _,
            );
        }

        // Configure PLL -
        // Planck has 16MHz external crystal
        pll.pllcsr.write(|w| w.pindiv().set_bit());
        pll.pllfrq
            .write(|w| w.pdiv().mhz96().plltm().factor_15().pllusb().set_bit());

        pll.pllcsr.modify(|_, w| w.plle().set_bit());
        while pll.pllcsr.read().plock().bit_is_clear() {}

        let write_lines = Direct((
            pd4.into_opendrain_high(),
            pd7.into_opendrain_high(),
            pe6.into_opendrain_high(),
            pb4.into_opendrain_high(),
            pb5.into_opendrain_high(),
        ));
        let read_lines = Direct((
            pf4.into_pull_up_input(),
            pf5.into_pull_up_input(),
            pf6.into_pull_up_input(),
            pf7.into_pull_up_input(),
            pb1.into_pull_up_input(),
            pb3.into_pull_up_input(),
            pb2.into_pull_up_input(),
        ));
        let scanner = Scanner::new(write_lines, read_lines, scan_delay);

        static mut USB_BUS: Option<UsbBusAllocator<UsbBus>> = None;
        let usb_bus: &'static UsbBusAllocator<UsbBus> =
            unsafe { USB_BUS.insert(UsbBus::new(usb_device)) };

        // USB device info copied from QMK's planck configuration:
        let uplink = UsbHid::new(usb_bus, |bus| {
            UsbDeviceBuilder::new(bus, UsbVidPid(0xcb10, 0x2157))
                .manufacturer("Keebio")
                .product("The Viterbi Keyboard")
                .device_release(0x0002)
                .build()
        });

        let backlight = Backlight::new(pb6, tc1);

        let _status = pe6.into_output_high();

        Self {
            scanner,
            uplink,
            backlight,
        }
    }
}

/// Initialize the keyboard, taking ownership of only the peripherals
/// necessary.
///
/// # Example
///
/// ```no_run
/// use polybius_viterbi::rev2::ViterbiRev2;
///
/// let peripherals = atmega_hal::Peripherals::take().unwrap();
/// let pins = atmega_hal::pins!(peripherals);
/// let keyboard: ViterbiRev2 = polybius_viterbi::rev2::from_parts!(peripherals, pins);
///
/// // Can still take other parts:
/// let tc1 = peripherals.TC3;
/// let pc6 = pins.pc6;
/// ```
#[macro_export]
macro_rules! viterbi_rev2 {
    ($dp:expr, $pins:expr) => {
        $crate::rev2::ViterbiRev2::from_parts(
            $dp.PLL,
            $dp.TC1,
            $dp.USB_DEVICE,
            $pins.pb1,
            $pins.pb2,
            $pins.pb3,
            $pins.pb4,
            $pins.pb5,
            $pins.pb6,
            $pins.pd0,
            $pins.pd1,
            $pins.pd4,
            $pins.pd7,
            $pins.pe6,
            $pins.pf4,
            $pins.pf5,
            $pins.pf6,
            $pins.pf7,
        )
    };
}
#[doc(inline)]
pub use crate::viterbi_rev2 as from_parts;
