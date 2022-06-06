//! Software serial for AVR systems using a single digital pin.

use core::{arch::asm, marker::PhantomData, mem};

use atmega_hal::{
    clock::Clock,
    delay::Delay,
    port::{
        mode::{Floating, Input, Output, PullUp},
        Pin,
    },
};
use avr_hal_generic::{
    avr_device::interrupt::{self, CriticalSection},
    port::PinOps,
};
use embedded_hal::{
    blocking::delay::DelayUs,
    digital::v2::{OutputPin, PinState},
};

/// GPIO pins that can be used with soft-serial.
///
/// # Safety
///
/// The routines provided by this trait are timing-critical. The implementations
/// of this trait provided in this crate are hand-written in assembly to ensure
/// that the timing of the read and write methods match.
///
/// You should not need to implement this yourself - unless your microcontroller
/// is not yet supported, in which case, please open an issue and/or pull
/// request so other people can also benefit!
pub unsafe trait SoftSerialPin: PinOps + Sized {
    fn read_byte(
        pin: &Pin<Input<PullUp>, Self>,
        cs: &CriticalSection,
        delay: u8,
    ) -> Result<(u8, bool), TransactionError>;

    fn write_byte(
        pin: &mut Pin<Output, Self>,
        cs: &CriticalSection,
        delay: u8,
        byte: u8,
        continuing: bool,
    );
}

macro_rules! impl_soft_serial_pin {
    (@debug_clk) => {
        // Toggle debug clock on PD1; to be used inside an asm! macro.
        // Used to be able to inspect read/write timing using a logic analyzer.
        // NOTE - this should not be committed uncommented!
        // "sbi 0x09, 1"
        ""
    };

    ($( #[$cfg:meta] $pin_type:ty {
        // The I/O address of the PINx register for this pin.
        pinx: $pinx:literal ,
        // The I/O address of the PORTx register for this pin.
        portx: $portx:literal ,
        // The index of the bit corresponding to this pin in the PORTx register.
        pin_bit: $pin_bit:literal ,
    } )*) => {$(
        #[$cfg]
        unsafe impl SoftSerialPin for $pin_type {
            fn read_byte(pin: &Pin<Input<PullUp>, Self>, cs: &CriticalSection, delay: u8) -> Result<(u8, bool), TransactionError> {
                let _ = (pin, cs);

                let byte: u8;
                // Bit 0: parity
                // Bit 1: idle line
                // Bit 2: continuing
                let flags: u8;

                unsafe { asm!(
                    "clr {flags}",
                    concat!("sbis ", $pinx, ", ", $pin_bit),
                    "rjmp 2f",
                        // Line is high (idle state)
                        "ori {flags}, 0b10",
                        "rjmp 3f",
                    "2:",
                        // Wait for next high transition (sync bit)
                        "0:",
                            concat!("sbis ", $pinx, ", ", $pin_bit),
                            "rjmp 0b",
                        impl_soft_serial_pin!(@debug_clk),

                        // Half-bit delay (align in-between transitions):
                        "mov {counter}, {delay}",
                        "lsr {counter}",
                        "0:",
                            "dec {counter}",
                            "brne 0b",

                        "clr {byte}",
                        "clr {bit}",

                        // Loop body: 6 cycles (not counting bit delays and NOPs)
                        "ldi {idx}, 9",
                        "0:",
                            // Bit delay
                            "mov {counter}, {delay}",
                            "1:",
                                "dec {counter}",
                                "brne 1b",

                            // 1 cycle padding to match write loop duration
                            "nop",

                            // N.B. out-of-order: the last bit read is the
                            // parity bit and so it shouldn't be appended to the
                            // output byte. Reordering these two instructions to
                            // the top of the loop prevents that from happening.
                            // On the first iteration, {byte} and {bit} are
                            // zero, so these instructions effectively don't do
                            // anything.
                            "lsl {byte}",
                            "or {byte}, {bit}",

                            "clr {bit}",
                            impl_soft_serial_pin!(@debug_clk),
                            concat!("sbic ", $pinx, ", ", $pin_bit),
                            "inc {bit}",
                            "eor {flags}, {bit}",

                        "dec {idx}",
                        "brne 0b",

                        // Bit delay
                        "mov {counter}, {delay}",
                        "0:",
                            "dec {counter}",
                            "brne 0b",

                        // N.B. continuing is true when line is _low_
                        impl_soft_serial_pin!(@debug_clk),
                        concat!("sbis ", $pinx, ", ", $pin_bit),
                        "ori {flags}, 0b100",
                    "3:",

                    idx = out(reg_upper) _,
                    counter = out(reg) _,
                    bit = out(reg) _,

                    delay = in(reg) delay,
                    byte = out(reg) byte,
                    flags = out(reg) flags,
                )}

                let parity = (flags & 0b1);
                let idle = (flags & 0b10);
                let continuing = (flags & 0b100);

                if idle != 0 {
                    Err(TransactionError::Idle)
                } else if parity != 0 {
                    Err(TransactionError::ParityError)
                } else {
                    Ok((byte, continuing != 0))
                }
            }

            fn write_byte(pin: &mut Pin<Output, Self>, cs: &CriticalSection, delay: u8, byte: u8, continuing: bool) {
                let _ = (pin, cs);

                unsafe { asm!(
                    // Sync bit
                    concat!("sbi ", $portx, ", ", $pin_bit),
                    impl_soft_serial_pin!(@debug_clk),

                    "clr {parity}",

                    // Loop body: 7 cycles (not counting bit delays and NOPs)
                    "ldi {idx}, 8",
                    "0:",
                        // Bit delay
                        "mov {counter}, {delay}",
                        "1:",
                            "dec {counter}",
                            "brne 1b",

                        // Set/clear bit in PORTx corresponding to MSB of {byte}
                        // Uniform delay: 5 cycles in either path
                        "sbrc {byte}, 7",
                        concat!("sbi ", $portx, ", ", $pin_bit),
                        "sbrs {byte}, 7",
                        concat!("cbi ", $portx, ", ", $pin_bit),
                        impl_soft_serial_pin!(@debug_clk),

                        // Update parity (don't care about lower 7 bits, just MSB)
                        "eor {parity}, {byte}",

                        "lsl {byte}",
                    "dec {idx}",
                    "brne 0b",

                    // Bit delay
                    "mov {counter}, {delay}",
                    "0:",
                        "dec {counter}",
                        "brne 0b",

                    // Write parity bit
                    "sbrc {parity}, 7",
                    concat!("sbi ", $portx, ", ", $pin_bit),
                    "sbrs {parity}, 7",
                    concat!("cbi ", $portx, ", ", $pin_bit),
                    impl_soft_serial_pin!(@debug_clk),

                    // Bit delay
                    "mov {counter}, {delay}",
                    "0:",
                        "dec {counter}",
                        "brne 0b",

                    // Reset line
                    "sbrc {continuing}, 0",
                    concat!("cbi ", $portx, ", ", $pin_bit),
                    "sbrs {continuing}, 0",
                    concat!("sbi ", $portx, ", ", $pin_bit),
                    impl_soft_serial_pin!(@debug_clk),

                    // Bit delay (min delay between this word and next sync)
                    "mov {counter}, {delay}",
                    "0:",
                        "dec {counter}",
                        "brne 0b",

                    idx = out(reg_upper) _,
                    counter = out(reg) _,
                    parity = out(reg) _,

                    delay = in(reg) delay,
                    continuing = in(reg) continuing as u8,
                    byte = inout(reg) byte => _,
                )}
            }
        }
    )*};
}

impl_soft_serial_pin! {
    #[cfg(feature = "atmega32u4")] atmega_hal::port::PD0 {
        pinx: 0x09,
        portx: 0x0b,
        pin_bit: 0,
    }
    #[cfg(feature = "atmega32u4")] atmega_hal::port::PD1 {
        pinx: 0x09,
        portx: 0x0b,
        pin_bit: 1,
    }
    #[cfg(feature = "atmega32u4")] atmega_hal::port::PD2 {
        pinx: 0x09,
        portx: 0x0b,
        pin_bit: 2,
    }
    #[cfg(feature = "atmega32u4")] atmega_hal::port::PD3 {
        pinx: 0x09,
        portx: 0x0b,
        pin_bit: 3,
    }
}

pub trait Baudrate<Clk> {
    /// Number of loop iterations needed for the delay of 1 bit.
    /// Assume a 3-cycle `dec; brne` loop.
    const DELAY_ITERS: u8;
}

macro_rules! impl_baudrate {
    ($( $t:ty { baud_rate: $baud_rate:literal } )*) => {$(
        impl<Clk: Clock> Baudrate<Clk> for $t {
            const DELAY_ITERS: u8 = (Clk::FREQ / $baud_rate / 3) as u8;
        }
    )*}
}

pub struct Baud100k;
pub struct Baud50k;

impl_baudrate! {
    Baud100k { baud_rate: 100_000 }
    Baud50k { baud_rate: 50_000 }
}

// High-level transaction procotol implemented using SoftSerialPin.
// TODO: Need more graceful handling of parity errors, timeouts, desync.
pub struct Serial<D, Clk, B> {
    pin: Bidir<D>,
    delay: Delay<Clk>,
    _typestate: PhantomData<(Clk, B)>,
}

impl<D: SoftSerialPin, Clk: Clock, B: Baudrate<Clk>> Serial<D, Clk, B>
where
    D: SoftSerialPin,
    Clk: Clock,
    B: Baudrate<Clk>,
    Delay<Clk>: DelayUs<u8>,
{
    pub fn new(pin: Pin<Input<Floating>, D>) -> Self {
        Self {
            pin: Bidir::Input(pin.into_pull_up_input()),
            delay: Delay::new(),
            _typestate: PhantomData,
        }
    }

    pub fn send_transaction(
        &mut self,
        request: &[u8],
        response: &mut [u8],
    ) -> Result<usize, TransactionError> {
        let result = interrupt::free(|cs| {
            // Pulse low to notify receiver
            self.pin.make_output(PinState::Low);
            self.delay.delay_us(10);
            let input = self.pin.make_input();

            // Wait for receiver to respond
            while input.is_high() {}
            while input.is_low() {}
            self.delay.delay_us(10);

            self.send_packet(cs, request);
            let bytes_received = self.recv_packet(cs, response)?;
            Ok(bytes_received)
        });
        self.pin.make_input();
        result
    }

    pub fn recv_transaction(&mut self) -> Result<(), TransactionError> {
        let result = interrupt::free(|cs| {
            let input = self.pin.make_input();
            // Wait for pulse to finish
            while input.is_low() {}
            self.delay.delay_us(10);

            // Send ACK pulse
            self.pin.make_output(PinState::Low);
            self.delay.delay_us(10);
            self.pin.make_input();

            // Stand-in echo implementation.
            //TODO external interface for request handling.
            let mut data = [0u8; 16];
            let bytes_read = self.recv_packet(cs, &mut data)?;
            self.send_packet(cs, &data[..bytes_read]);
            Ok(())
        });
        self.pin.make_input();
        result
    }

    fn send_packet(&mut self, cs: &CriticalSection, data: &[u8]) {
        // Pull line low (SOT)
        let mut output = self.pin.make_output(PinState::Low);
        self.delay.delay_us(10);

        for (index, &byte) in data.iter().enumerate() {
            SoftSerialPin::write_byte(
                &mut output,
                cs,
                B::DELAY_ITERS,
                byte,
                index < data.len() - 1,
            );
        }
        self.pin.make_input();
    }

    fn recv_packet(
        &mut self,
        cs: &CriticalSection,
        data: &mut [u8],
    ) -> Result<usize, TransactionError> {
        // Wait for line pulled low (SOT)
        let input = self.pin.make_input();
        while input.is_high() {}

        let mut index = 0;
        loop {
            let (byte, continuing) = SoftSerialPin::read_byte(&input, cs, B::DELAY_ITERS)?;
            data[index] = byte;
            index += 1;
            if !continuing {
                break;
            } else if index >= data.len() {
                return Err(TransactionError::BufferOverflow);
            }
        }
        Ok(index)
    }
}

pub enum TransactionError {
    BufferOverflow,
    ParityError,
    Idle,
}

enum Bidir<D> {
    Invalid,
    Input(Pin<Input<PullUp>, D>),
    Output(Pin<Output, D>),
}

impl<D: PinOps> Bidir<D> {
    pub fn make_input(&mut self) -> &Pin<Input<PullUp>, D> {
        match self {
            Self::Invalid => unreachable!(),
            Self::Input(x) => x,
            Self::Output(_) => {
                let output_pin = self.take().into_output().unwrap();
                *self = Self::Input(output_pin.into_pull_up_input());
                self.as_input().unwrap()
            }
        }
    }

    pub fn make_output(&mut self, state: PinState) -> &mut Pin<Output, D> {
        match self {
            Self::Invalid => unreachable!(),
            Self::Input(_) => {
                let input_pin = self.take().into_input().unwrap();
                let output_pin = match state {
                    PinState::Low => input_pin.into_output(),
                    PinState::High => input_pin.into_output_high(),
                };
                *self = Self::Output(output_pin);
                self.as_output().unwrap()
            }
            Self::Output(x) => {
                x.set_state(state).unwrap();
                x
            }
        }
    }

    fn take(&mut self) -> Self {
        mem::replace(self, Self::Invalid)
    }

    fn into_input(self) -> Option<Pin<Input<PullUp>, D>> {
        match self {
            Self::Input(x) => Some(x),
            _ => None,
        }
    }

    fn into_output(self) -> Option<Pin<Output, D>> {
        match self {
            Self::Output(x) => Some(x),
            _ => None,
        }
    }

    fn as_input(&self) -> Option<&Pin<Input<PullUp>, D>> {
        match self {
            Self::Input(x) => Some(x),
            _ => None,
        }
    }

    fn as_output(&mut self) -> Option<&mut Pin<Output, D>> {
        match self {
            Self::Output(x) => Some(x),
            _ => None,
        }
    }
}
