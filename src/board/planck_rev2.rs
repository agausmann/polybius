use crate::diodes::ColToRow;
use crate::keyboard::Keyboard;
use crate::scanner::{Direct, ScanMatrix};
use crate::uplink::usb::UsbHid;
use atmega_hal::port::mode::Output;
use atmega_hal::port::PB7;
use atmega_hal::{
    clock::MHz16,
    delay::Delay,
    pac::Peripherals,
    port::mode::{Input, OpenDrain, PullUp},
    port::{Pin, PB0, PB4, PB5, PB6, PC7, PD0, PD4, PD5, PD6, PD7, PF0, PF1, PF4, PF5, PF6, PF7},
};
use atmega_usbd::UsbBus;
use core::arch::asm;
use embedded_hal::blocking::delay::DelayUs;
use usb_device::bus::UsbBusAllocator;
use usb_device::device::{UsbDeviceBuilder, UsbVidPid};

pub const ROWS: usize = 4;
pub const COLS: usize = 12;

pub type ClockSpeed = MHz16;

pub type WriteLines = Direct<(
    Pin<OpenDrain, PD0>,
    Pin<OpenDrain, PD5>,
    Pin<OpenDrain, PB5>,
    Pin<OpenDrain, PB6>,
)>;
pub type ReadLines = Direct<(
    Pin<Input<PullUp>, PF1>,
    Pin<Input<PullUp>, PF0>,
    Pin<Input<PullUp>, PB0>,
    Pin<Input<PullUp>, PC7>,
    Pin<Input<PullUp>, PF4>,
    Pin<Input<PullUp>, PF5>,
    Pin<Input<PullUp>, PF6>,
    Pin<Input<PullUp>, PF7>,
    Pin<Input<PullUp>, PD4>,
    Pin<Input<PullUp>, PD6>,
    Pin<Input<PullUp>, PB4>,
    Pin<Input<PullUp>, PD7>,
)>;
pub type ScanDelay = fn();
pub type Diodes = ColToRow;
pub type Scanner = ScanMatrix<WriteLines, ReadLines, ScanDelay, Diodes, ROWS, COLS>;

fn scan_delay() {
    let mut delay = Delay::<ClockSpeed>::new();
    delay.delay_us(30_u8);
}

pub type Uplink = UsbHid<'static, UsbBus>;

// TODO variable brightness with PWM
pub struct Backlight {
    pin: Pin<Output, PB7>,
}

impl crate::backlight::Backlight for Backlight {
    fn num_levels(&self) -> u8 {
        2
    }

    fn level(&self) -> u8 {
        if self.pin.is_set_low() {
            0
        } else {
            1
        }
    }

    fn set_level(&mut self, level: u8) {
        if level == 0 {
            self.pin.set_low();
        } else {
            self.pin.set_high();
        }
    }
}

pub struct PlanckRev2 {
    scanner: Scanner,
    uplink: Uplink,
    backlight: Backlight,
}

impl Keyboard<ROWS, COLS> for PlanckRev2 {
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

impl PlanckRev2 {
    pub fn new(dp: Peripherals) -> Self {
        // Disable JTAG functionality to gain control of pins PF4-PF7.
        // This procedure has tight timing requirements (4 cycles between writes)
        // which can't be guaranteed by the codegen/linker with the safe code:
        // dp.JTAG.mcucr.modify(|_, w| w.jtd().set_bit());
        // dp.JTAG.mcucr.modify(|_, w| w.jtd().set_bit());
        unsafe {
            asm!(
                "in r25, 0x35",
                "ori r25, 0x80",
                "out 0x35, r25",
                "out 0x35, r25",
                out("r25") _,
            );
        }
        let pins = atmega_hal::pins!(dp);

        // Configure PLL -
        // Planck has 16MHz external crystal
        dp.PLL.pllcsr.write(|w| w.pindiv().set_bit());
        dp.PLL
            .pllfrq
            .write(|w| w.pdiv().mhz96().plltm().factor_15().pllusb().set_bit());

        dp.PLL.pllcsr.modify(|_, w| w.plle().set_bit());
        while dp.PLL.pllcsr.read().plock().bit_is_clear() {}

        let write_lines = Direct((
            pins.pd0.into_opendrain_high(),
            pins.pd5.into_opendrain_high(),
            pins.pb5.into_opendrain_high(),
            pins.pb6.into_opendrain_high(),
        ));
        let read_lines = Direct((
            pins.pf1.into_pull_up_input(),
            pins.pf0.into_pull_up_input(),
            pins.pb0.into_pull_up_input(),
            pins.pc7.into_pull_up_input(),
            pins.pf4.into_pull_up_input(),
            pins.pf5.into_pull_up_input(),
            pins.pf6.into_pull_up_input(),
            pins.pf7.into_pull_up_input(),
            pins.pd4.into_pull_up_input(),
            pins.pd6.into_pull_up_input(),
            pins.pb4.into_pull_up_input(),
            pins.pd7.into_pull_up_input(),
        ));
        let scanner = Scanner::new(write_lines, read_lines, scan_delay);

        static mut USB_BUS: Option<UsbBusAllocator<UsbBus>> = None;
        let usb_bus: &'static UsbBusAllocator<UsbBus> =
            unsafe { USB_BUS.insert(UsbBus::new(dp.USB_DEVICE)) };

        // USB device info copied from QMK's planck configuration:
        let uplink = UsbHid::new(usb_bus, |bus| {
            UsbDeviceBuilder::new(bus, UsbVidPid(0x03a8, 0xae01))
                .manufacturer("OLKB")
                .product("Planck")
                .device_release(0x0002)
                .build()
        });

        let backlight = Backlight {
            pin: pins.pb7.into_output(),
        };

        Self {
            scanner,
            uplink,
            backlight,
        }
    }
}
