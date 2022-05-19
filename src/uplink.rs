use crate::keycode::{KeyAction, Keycode};

pub trait Uplink {
    type Error;

    fn poll(&mut self) -> Result<(), Self::Error>;

    fn key_event(&mut self, keycode: Keycode, action: KeyAction) -> Result<(), Self::Error>;
}

#[cfg(feature = "usb")]
pub mod usb {
    use super::Uplink;
    use crate::keycode::{HidKeycode, KeyAction, Keycode};
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
        pending: bool,
    }

    impl<'a, B> UsbHid<'a, B>
    where
        B: UsbBus,
    {
        pub fn new<DeviceBuilder>(
            alloc: &'a UsbBusAllocator<B>,
            device_builder: DeviceBuilder,
        ) -> Self
        where
            DeviceBuilder: for<'b> FnOnce(&'b UsbBusAllocator<B>) -> UsbDevice<'b, B>,
        {
            let hid = HIDClass::new(alloc, KeyboardReport::desc(), 10);
            let device = device_builder(alloc);

            Self {
                device,
                hid,
                report: KeyboardReport {
                    modifier: 0,
                    reserved: 0,
                    leds: 0,
                    keycodes: [0; 6],
                },
                pending: false,
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
                if self.hid.pull_raw_output(&mut report).is_ok() {}
            }
            if self.pending {
                if self.hid.push_input(&self.report).is_ok() {
                    self.pending = false;
                }
            }
            Ok(())
        }

        fn key_event(&mut self, keycode: Keycode, action: KeyAction) -> Result<(), Self::Error> {
            let hid_keycode = match keycode {
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

            if let Some(modifier) = modifier {
                match action {
                    KeyAction::Pressed => {
                        self.report.modifier |= 1 << modifier;
                    }
                    KeyAction::Released => {
                        self.report.modifier &= !(1 << modifier);
                    }
                }
                self.pending = true;
            } else {
                let raw_keycode = hid_keycode as u8;

                match action {
                    KeyAction::Pressed => {
                        for slot in &mut self.report.keycodes {
                            if *slot == raw_keycode {
                                break;
                            } else if *slot == 0 {
                                *slot = raw_keycode;
                                self.pending = true;
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
                                self.pending = true;
                                break;
                            }
                        }
                    }
                }
            }
            Ok(())
        }
    }
}
