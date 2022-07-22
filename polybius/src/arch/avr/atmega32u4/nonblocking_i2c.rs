use core::marker::PhantomData;

use atmega_hal::{
    clock::Clock,
    pac::{
        twi::{twcr, TWCR},
        TWI,
    },
    port::{
        mode::{Floating, Input},
        Pin, PD0, PD1,
    },
};

/// Low-level driver for the TWI peripheral.
///
/// # How to use
///
/// This driver is designed for the layer defined in sections 20.6 "Using
/// the TWI" and 20.7 "Transmission Modes" of the [ATmega16u4 / ATmega32u4
/// datasheet][ds].
///
/// It provides a convenient API to query and match against the statuses defined
/// in Tables 20-1 through 20-5, and also methods that should be used to
/// implement the possible responses for each status as defined in the
/// "Application Software Response" column of those tables.
///
/// [ds]: https://ww1.microchip.com/downloads/en/devicedoc/atmel-7766-8-bit-avr-atmega16u4-32u4_datasheet.pdf
pub struct Twi<CLOCK> {
    twi: TWI,
    _clk: PhantomData<CLOCK>,
}

impl Twi
where
    CLOCK: Clock,
{
    pub fn new(
        twi: TWI,
        sda: Pin<Input<Floating>, PD1>,
        scl: Pin<Input<Floating>, PD0>,
        bitrate: u8,
    ) -> Self {
        let _ = (sda, scl);

        // Setup bit rate
        let mut twbr = ((CLOCK::FREQ / bitrate) - 16) / 2;
        let mut twps = 0;
        while twbr > u8::MAX as u32 && twps < 3 {
            twbr /= 4;
            twps += 1;
        }
        twi.twbr
            .write(|w| unsafe { w.bits(twbr.try_into().unwrap_or(255)) });
        twi.twsr.write(|w| w.twps().bits(twps));

        Self {
            twi,
            _clk: PhantomData,
        }
    }

    /// Get the current status reported by the TWI peripheral.
    ///
    /// # Errors
    ///
    /// Returns an `Err` containing the raw status register value if
    /// it is an unknown value.
    pub fn status(&self) -> Result<Status, u8> {
        let reg = self.twi.twsr.read().bits();
        Status::from_reg(reg).ok_or(reg)
    }

    /// Writes SLA+R/W or a data byte to be transmitted.
    pub fn write(&self, byte: u8) {
        self.twi.twdr.write(|w| unsafe { w.bits(byte) });
    }

    /// Reads the last byte transmitted on the bus.
    pub fn read(&self) -> u8 {
        self.twi.twdr.read().bits()
    }

    /// Writes STA=0, STO=0, TWINT=1 to TWCR.
    pub fn control_resume(&self) {
        self.twi
            .twcr
            .modify(|_, w| w.twsta().clear_bit().twsto().clear_bit().twint().set_bit());
    }

    /// Writes STA=1, STO=0, TWINT=1 to TWCR.
    ///
    /// A START or repeated START condition will be transmitted.
    pub fn control_start(&self) {
        self.twi
            .twcr
            .modify(|_, w| w.twsta().set_bit().twsto().clear_bit().twint().set_bit());
    }

    /// Writes STA=0, STO=1, TWINT=1 to TWCR.
    ///
    /// A STOP condition will be transmitted.
    pub fn control_stop(&self) {
        self.twi
            .twcr
            .modify(|_, w| w.twsta().clear_bit().twsto().set_bit().twint().set_bit());
    }

    /// Writes STA=1, STO=1, TWINT=1 to TWCR.
    ///
    /// A STOP condition followed by a START condition will be transmitted.
    pub fn control_stop_start(&self) {
        self.twi
            .twcr
            .modify(|_, w| w.twsta().set_bit().twsto().set_bit().twint().set_bit())
    }

    /// Writes STA=0, STO=0, TWINT=1, TWEA=1 to TWCR.
    ///
    /// ACK will be returned for the next data byte or matching SLA+R/W received.
    pub fn control_ack(&self) {
        self.twi.twcr.modify(|_, w| {
            w.twsta()
                .clear_bit()
                .twsto()
                .clear_bit()
                .twint()
                .set_bit()
                .twea()
                .set_bit()
        });
    }

    /// Writes STA=0, STO=0, TWINT=1, TWEA=0 to TWCR.
    ///
    /// NOT ACK will be returned for the next data byte or matching SLA+R/W received.
    pub fn control_nack(&self) {
        self.twi.twcr.modify(|_, w| {
            w.twsta()
                .clear_bit()
                .twsto()
                .clear_bit()
                .twint()
                .set_bit()
                .twea()
                .set_bit()
        });
    }
}

/// Status codes reported by the TWI peripheral.
///
/// To get the current status, see [`Twi::status`]
pub enum Status {
    /// 0x08 - A START condition has been transmitted
    ///
    /// # Responses
    ///
    /// - `write(sla_rw)` followed by `control_resume()` - SLA+R/W will be
    ///   transmitted; ACK or NOT ACK will be received
    Start,

    /// 0x10 - A repeated START condition has been transmitted
    ///
    /// # Responses
    ///
    /// - `write(sla_rw)` followed by `control_resume()` - SLA+R/W will be
    ///   transmitted; ACK or NOT ACK will be received
    RepeatedStart,

    /// 0x18 - SLA+W has been transmitted; ACK has been received
    ///
    /// # Responses
    ///
    /// - `write(data_byte)` followed by `control_resume()` - Data byte will be
    ///   transmitted and ACK or NOT ACK will be received
    ///
    /// - `control_start()` - Repeated START will be transmitted
    ///
    /// - `control_stop()` - STOP condition will be transmitted
    ///
    /// - `control_stop_start()` - STOP condition followed by a START condition
    ///   will be transmitted
    MasterAddrWriteAck,

    /// 0x20 - SLA+W has been transmitted; NOT ACK has been received
    ///
    /// # Responses
    ///
    /// - `write(data_byte)` followed by `control_resume()` - Data byte will be
    ///   transmitted and ACK or NOT ACK will be received
    ///
    /// - `control_start()` - Repeated START will be transmitted
    ///
    /// - `control_stop()` - STOP condition will be transmitted
    ///
    /// - `control_stop_start()` - STOP condition followed by a START condition
    ///   will be transmitted
    MasterAddrWriteNack,

    /// 0x28 - Data byte has been transmitted; ACK has been received
    ///
    /// # Responses
    ///
    /// - `write(data_byte)` followed by `control_resume()` - Data byte will be
    ///   transmitted and ACK or NOT ACK will be received
    ///
    /// - `control_start()` - Repeated START will be transmitted
    ///
    /// - `control_stop()` - STOP condition will be transmitted
    ///
    /// - `control_stop_start()` - STOP condition followed by a START condition
    ///   will be transmitted
    MasterDataWriteAck,

    /// 0x30 - Data byte has been transmitted; NOT ACK has been received
    ///
    /// # Responses
    ///
    /// - `write(data_byte)` followed by `control_resume()` - Data byte will be
    ///   transmitted and ACK or NOT ACK will be received
    ///
    /// - `control_start()` - Repeated START will be transmitted
    ///
    /// - `control_stop()` - STOP condition will be transmitted
    ///
    /// - `control_stop_start()` - STOP condition followed by a START condition
    ///   will be transmitted
    MasterDataWriteNack,

    /// 0x38 - Arbitration lost in SLA+W or data bytes
    ///
    /// # Responses
    ///
    /// - `control_resume()` - 2-wire Serial Bus will be released and not
    ///   addressed Slave mode entered
    ///
    /// - `control_start()` - A START condition will be transmitted when the bus
    ///   becomes free
    ArbitrationLost,

    /// 0x40 - SLA+R has been transmitted; ACK has been received
    ///
    /// # Responses
    ///
    /// - `control_nack()` - Data byte will be received and NOT ACK will be
    ///   returned
    ///
    /// - `control_ack()` - Data byte will be received and ACK will be returned
    MasterAddrReadAck,

    /// 0x48 - SLA+R has been transmitted; NOT ACK has been received
    ///
    /// # Responses
    ///
    /// - `control_start()` - Repeated START will be transmitted
    ///
    /// - `control_stop()` - STOP condition will be transmitted
    ///
    /// - `control_stop_start()` - STOP condition followed by a START condition
    ///   will be transmitted
    MasterAddrReadNack,

    /// 0x50 - Data byte has been received; ACK has been returned
    ///
    /// # Responses
    ///
    /// - `control_nack()` - Data byte will be received and NOT ACK will be
    ///   returned
    ///
    /// - `control_ack()` - Data byte will be received and ACK will be returned
    MasterDataReadAck,

    /// 0x58 - Data byte has been received; NOT ACK has been returned
    ///
    /// # Responses
    ///
    /// - `control_start()` - Repeated START will be transmitted
    ///
    /// - `control_stop()` - STOP condition will be transmitted
    ///
    /// - `control_stop_start()` - STOP condition followed by a START condition
    ///   will be transmitted
    MasterDataReadNack,

    /// 0x60 - Own SLA+W has been received; ACK has been returned
    ///
    /// # Responses
    ///
    /// - `control_nack()` - Data byte will be received and NOT ACK will be
    ///   returned
    ///
    /// - `control_ack()` - Data byte will be received and ACK will be returned
    SlaveAddrWriteAck,

    /// 0x68 - Arbitration lost in SLA+R/W as Master; own SLA+W has been
    /// received; ACK has been returned
    ///
    /// # Responses
    ///
    /// - `control_nack()` - Data byte will be received and NOT ACK will be
    ///   returned
    ///
    /// - `control_ack()` - Data byte will be received and ACK will be returned
    ArbitrationLostAndSlaveAddrWriteAck,

    /// 0x70 - General call address has been received; ACK has been returned
    ///
    /// # Responses
    ///
    /// - `control_nack()` - Data byte will be received and NOT ACK will be
    ///   returned
    ///
    /// - `control_ack()` - Data byte will be received and ACK will be returned
    GeneralCallAck,

    /// 0x78 - Arbitration lost in SLA+R/W as Master; General Call address has
    /// been received; ACK has been returned
    ///
    /// # Responses
    ///
    /// - `control_nack()` - Data byte will be received and NOT ACK will be
    ///   returned
    ///
    /// - `control_ack()` - Data byte will be received and ACK will be returned
    ArbitrationLostAndGeneralCallAck,

    /// 0x80 - Previously addressed with own SLA+W; data has been received; ACK
    /// has been returned
    ///
    /// Received data byte can be read with `read()`.
    ///
    /// # Responses
    ///
    /// - `control_nack()` - Data byte will be received and NOT ACK will be
    ///   returned
    ///
    /// - `control_ack()` - Data byte will be received and ACK will be returned
    SlaveDataWriteAck,

    /// 0x88 - Previously addressed with own SLA+W; data has been received; NOT
    /// ACK has been returned
    ///
    /// Received data byte can be read with `read()`.
    ///
    /// # Responses
    ///
    /// - `control_nack()` - Switched to the not addressed
    ///   Slave mode; no recognition of own SLA or GCA
    ///
    /// - `control_ack()` - Switched to the not addressed
    ///   Slave mode; own SLA will be recognized; GCA will be recognized if
    ///   TWGCE = "1"
    ///
    /// - `control_nack_start()` - Switched to the not
    ///   addressed Slave mode; no recognition of own SLA or GCA; a START
    ///   condition will be transmitted when the bus becomes free
    ///
    /// - `control_ack_start()` - Switched to the not
    ///   addressed Slave mode; own SLA will be recognized; GCA will be
    ///   recognized if TWGCE = "1"; a START condition will be transmitted when
    ///   the bus becomes free
    SlaveDataWriteNack,

    /// 0x90 - Previously addressed with general call; data has been received;
    /// ACK has been returned
    ///
    /// Received data byte can be read with `read()`.
    ///
    /// # Responses
    ///
    /// - `control_nack()` - Data byte will be received and NOT ACK will be
    ///   returned
    ///
    /// - `control_ack()` - Data byte will be received and ACK will be returned
    GeneralCallDataWriteAck,

    /// 0x98 - Previously addressed with general call; data has been received;
    /// NOT ACK has been returned
    ///
    /// Received data byte can be read with `read()`.
    ///
    /// # Responses
    ///
    /// - `control_nack()` - Switched to the not addressed
    ///   Slave mode; no recognition of own SLA or GCA
    ///
    /// - `control_ack()` - Switched to the not addressed
    ///   Slave mode; own SLA will be recognized; GCA will be recognized if
    ///   TWGCE = "1"
    ///
    /// - `control_nack_start()` - Switched to the not
    ///   addressed Slave mode; no recognition of own SLA or GCA; a START
    ///   condition will be transmitted when the bus becomes free
    ///
    /// - `control_ack_start()` - Switched to the not
    ///   addressed Slave mode; own SLA will be recognized; GCA will be
    ///   recognized if TWGCE = "1"; a START condition will be transmitted when
    ///   the bus becomes free
    GeneralCallDataWriteNack,

    /// 0xA0 - A STOP condition or repeated START condition has been received
    /// while still addressed as Slave
    ///
    /// Received data byte can be read with `read()`.
    ///
    /// # Responses
    ///
    /// - `control_nack()` - Switched to the not addressed
    ///   Slave mode; no recognition of own SLA or GCA
    ///
    /// - `control_ack()` - Switched to the not addressed
    ///   Slave mode; own SLA will be recognized; GCA will be recognized if
    ///   TWGCE = "1"
    ///
    /// - `control_nack_start()` - Switched to the not
    ///   addressed Slave mode; no recognition of own SLA or GCA; a START
    ///   condition will be transmitted when the bus becomes free
    ///
    /// - `control_ack_start()` - Switched to the not
    ///   addressed Slave mode; own SLA will be recognized; GCA will be
    ///   recognized if TWGCE = "1"; a START condition will be transmitted when
    ///   the bus becomes free
    SlaveStopOrRepeatedStart,

    /// 0xA8 - Own SLA+R has been received; ACK has been returned
    ///
    /// # Responses
    ///
    /// - `write(data_byte)` followed by `control_nack()` - Last data byte will
    ///   be transmitted and NOT ACK should be received
    ///
    /// - `write(data_byte)` followed by `control_ack()` - Data byte will be
    ///   transmitted and ACK should be received
    SlaveAddrReadAck,

    /// 0xB0 - Arbitration lost in SLA+R/W as Master; own SLA+R has been
    /// received; ACK has been returned
    ///
    /// # Responses
    ///
    /// - `write(data_byte)` followed by `control_nack()` - Last data byte will
    ///   be transmitted and NOT ACK should be received
    ///
    /// - `write(data_byte)` followed by `control_ack()` - Data byte will be
    ///   transmitted and ACK should be received
    ArbitrationLostAndSlaveAddrReadAck,

    /// 0xB8 - Data byte in TWDR has been transmitted; ACK has been received
    ///
    /// # Responses
    ///
    /// - `write(data_byte)` followed by `control_nack()` - Last data byte will
    ///   be transmitted and NOT ACK should be received
    ///
    /// - `write(data_byte)` followed by `control_ack()` - Data byte will be
    ///   transmitted and ACK should be received
    SlaveDataReadAck,

    /// 0xC0 - Data byte in TWDR has been transmitted; NOT ACK has been received
    ///
    /// # Responses
    ///
    /// - `control_nack()` - Switched to the not addressed
    ///   Slave mode; no recognition of own SLA or GCA
    ///
    /// - `control_ack()` - Switched to the not addressed
    ///   Slave mode; own SLA will be recognized; GCA will be recognized if
    ///   TWGCE = "1"
    ///
    /// - `control_nack_start()` - Switched to the not
    ///   addressed Slave mode; no recognition of own SLA or GCA; a START
    ///   condition will be transmitted when the bus becomes free
    ///
    /// - `control_ack_start()` - Switched to the not
    ///   addressed Slave mode; own SLA will be recognized; GCA will be
    ///   recognized if TWGCE = "1"; a START condition will be transmitted when
    ///   the bus becomes free
    SlaveDataReadNack,

    /// 0xC8 - Last data byte in TWDR has been transmitted (TWEA = "0"); ACK
    /// has been recdeived
    ///
    /// # Responses
    ///
    /// - `control_nack()` - Switched to the not addressed
    ///   Slave mode; no recognition of own SLA or GCA
    ///
    /// - `control_ack()` - Switched to the not addressed
    ///   Slave mode; own SLA will be recognized; GCA will be recognized if
    ///   TWGCE = "1"
    ///
    /// - `control_nack_start()` - Switched to the not
    ///   addressed Slave mode; no recognition of own SLA or GCA; a START
    ///   condition will be transmitted when the bus becomes free
    ///
    /// - `control_ack_start()` - Switched to the not
    ///   addressed Slave mode; own SLA will be recognized; GCA will be
    ///   recognized if TWGCE = "1"; a START condition will be transmitted when
    ///   the bus becomes free
    SlaveFinalDataReadAck,

    /// 0xF8 - No relevant state information available; TWINT = "0"
    ///
    /// # Responses
    ///
    /// No action should be taken until another state is reported.
    None,

    /// 0x00 - Bus error due to an illegal START or STOP condition
    ///
    /// # Responses
    ///
    /// - `control_stop()` - Only the internal hardware is affected, no STOP
    ///   condition is sent on the bus. In all cases, the bus is released
    BusError,
}

impl Status {
    /// Convert a TWSR register value to a status code.
    fn from_reg(reg: u8) -> Option<Self> {
        match reg & 0xf8 {
            0x08 => Some(Self::Start),
            0x10 => Some(Self::RepeatedStart),
            0x18 => Some(Self::MasterAddrWriteAck),
            0x20 => Some(Self::MasterAddrWriteNack),
            0x28 => Some(Self::MasterDataWriteAck),
            0x30 => Some(Self::MasterDataWriteNack),
            0x38 => Some(Self::ArbitrationLost),
            0x40 => Some(Self::MasterAddrReadAck),
            0x48 => Some(Self::MasterAddrReadNack),
            0x50 => Some(Self::MasterDataReadAck),
            0x58 => Some(Self::MasterDataReadNack),
            0x60 => Some(Self::SlaveAddrWriteAck),
            0x68 => Some(Self::ArbitrationLostAndSlaveAddrWriteAck),
            0x70 => Some(Self::GeneralCallAck),
            0x78 => Some(Self::ArbitrationLostAndGeneralCallAck),
            0x80 => Some(Self::SlaveDataWriteAck),
            0x88 => Some(Self::SlaveDataWriteNack),
            0x90 => Some(Self::GeneralCallDataWriteAck),
            0x98 => Some(Self::GeneralCallDataWriteNack),
            0xa0 => Some(Self::SlaveStopOrRepeatedStart),
            0xa8 => Some(Self::SlaveAddrReadAck),
            0xb0 => Some(Self::ArbitrationLostAndSlaveAddrReadAck),
            0xb8 => Some(Self::SlaveDataReadAck),
            0xc0 => Some(Self::SlaveDataReadNack),
            0xc8 => Some(Self::SlaveFinalDataReadAck),
            0xf8 => Some(Self::None),
            0x00 => Some(Self::BusError),
            _ => None,
        }
    }
}
