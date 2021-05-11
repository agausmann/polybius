use crate::keycode::{HidKeycode, Keycode};

pub struct KeyEvent {
    pub keycode: Keycode,
    pub action: KeyAction,
}

pub enum KeyAction {
    Pressed,
    Released,
}

pub trait Uplink {
    type Error;

    fn poll(&mut self) -> Result<(), Self::Error>;

    fn send(&mut self, event: KeyEvent) -> Result<(), Self::Error>;
}

#[cfg(feature = "usb")]
pub mod usb {
    use super::*;
    use usb_device::bus::{UsbBus, UsbBusAllocator};
    use usb_device::device::UsbDevice;
    use usb_device::UsbError;
    use usbd_hid::descriptor::{KeyboardReport, SerializedDescriptor};
    use usbd_hid::hid_class::HIDClass;

    pub struct UsbHid<'a, B>
    where
        B: UsbBus,
    {
        device: UsbDevice<'a, B>,
        hid: HIDClass<'a, B>,
        report: KeyboardReport,
    }

    impl<'a, B> UsbHid<'a, B>
    where
        B: UsbBus,
    {
        pub fn new(device: UsbDevice<'a, B>, alloc: &'a UsbBusAllocator<B>) -> Self {
            let hid = HIDClass::new(alloc, KeyboardReport::desc(), 10);
            Self {
                device,
                hid,
                report: KeyboardReport {
                    modifier: 0,
                    leds: 0,
                    keycodes: [0; 6],
                },
            }
        }
    }

    impl<'a, B> Uplink for UsbHid<'a, B>
    where
        B: UsbBus,
    {
        type Error = UsbError;

        fn poll(&mut self) -> Result<(), Self::Error> {
            if self.device.poll(&mut [&mut self.hid]) {
                let mut report = [0u8; 1];
                self.hid.pull_raw_output(&mut report)?;
            }
            Ok(())
        }

        fn send(&mut self, event: KeyEvent) -> Result<(), Self::Error> {
            let hid_keycode = match event.keycode {
                Keycode::Hid(hid) => hid,
                _ => return Ok(()),
            };

            // Handle modifiers
            let modifier = match hid_keycode {
                HidKeycode::LeftControl => Some(0),
                HidKeycode::LeftShift => Some(1),
                HidKeycode::LeftAlt => Some(2),
                HidKeycode::LeftGui => Some(3),
                HidKeycode::RightControl => Some(4),
                HidKeycode::RightShift => Some(5),
                HidKeycode::RightAlt => Some(6),
                HidKeycode::RightGui => Some(7),
                _ => None,
            };

            let mut modified = false;

            if let Some(modifier) = modifier {
                match event.action {
                    KeyAction::Pressed => {
                        self.report.modifier |= 1 << modifier;
                    }
                    KeyAction::Released => {
                        self.report.modifier &= !(1 << modifier);
                    }
                }
                modified = true;
            } else {
                let raw_keycode = hid_keycode as u8;

                match event.action {
                    KeyAction::Pressed => {
                        for slot in &mut self.report.keycodes {
                            if *slot == raw_keycode {
                                break;
                            } else if *slot == 0 {
                                *slot = raw_keycode;
                                modified = true;
                                break;
                            }
                        }
                    }
                    KeyAction::Released => {
                        for i in 0..self.report.keycodes.len() {
                            if self.report.keycodes[i] == raw_keycode {
                                for j in (i + 1)..self.report.keycodes.len() {
                                    self.report.keycodes[j - 1] = self.report.keycodes[j];
                                }
                                self.report.keycodes[self.report.keycodes.len() - 1] = 0;
                                modified = true;
                                break;
                            }
                        }
                    }
                }
            }

            if modified {
                self.hid.push_input(&self.report)?;
            }
            Ok(())
        }
    }
}
