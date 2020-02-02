//! TCA9535 Low-Voltage 16-Bit I2C and SMBus Low-Power I/O Expander
//!
//! https://www.ti.com/lit/ds/symlink/tca9535.pdf

// Tests require std for mocking the i2c bus
#![cfg_attr(not(test), no_std)]

use embedded_hal::blocking::i2c::{Write, WriteRead};
use core::marker::PhantomData;
use core::convert::TryFrom;

pub struct Tca9535<T> {
    address: u8,
    i2c: PhantomData<T>,
}

impl<T,E> Tca9535<T>
where T: WriteRead<Error = E> + Write<Error = E>
{
    pub fn new(_i2c: &T, address: Address) -> Result<Self, E> {
        Ok(Self{address: address as u8, i2c: PhantomData})
    }

    pub fn address(&self) -> Address {
        match self.address {
            0x20 => Address::ADDR_0x20,
            0x21 => Address::ADDR_0x21,
            0x22 => Address::ADDR_0x22,
            0x23 => Address::ADDR_0x23,
            0x24 => Address::ADDR_0x24,
            0x25 => Address::ADDR_0x25,
            0x26 => Address::ADDR_0x26,
            0x27 => Address::ADDR_0x27,
            _ => unreachable!()
        }
    }

    /// The Input Port registers reflect the incoming logic levels of the pins, regardless of
    /// whether the pin is defined as an input or an output by the Configuration Register.
    pub fn read_inputs(&self, i2c: &mut T) -> Result<u16, E> {
        let mut buffer = [0u8;2];
        i2c.write_read(self.address, &[Register::INPUT_PORT0 as u8], &mut buffer)?;
        Ok(u16::from_le_bytes(buffer))
    }

    /// The Output Port registers show the outgoing logic levels of the pins defined as outputs
    /// by the Configuration Register.  These values reflect the state of the flip-flop controlling
    /// the output section, not the actual pin value.
    pub fn read_outputs(&self, i2c: &mut T) -> Result<u16, E> {
        let mut buffer = [0u8;2];
        i2c.write_read(self.address, &[Register::OUTPUT_PORT0 as u8], &mut buffer)?;
        Ok(u16::from_le_bytes(buffer))
    }

    /// Set the output state for all pins configured as output pins in the Configuration Register.
    /// Has no effect for pins configured as input pins.
    pub fn write_outputs(&self, i2c: &mut T, values: u16) -> Result<(), E> {
        let buffer = [Register::OUTPUT_PORT0 as u8, (values & 0x00FF) as u8, (values >> 8) as u8];
        i2c.write(self.address, &buffer)
    }

    /// Configure the direction of the I/O pins.  Bits set to 1 are configured as input pins with
    /// high-impedance output drivers.  Bits set to 0 are set as output pins.
    pub fn write_config(&self, i2c: &mut T, values: u16) -> Result<(), E> {
        let buffer = [Register::CONFIG_PORT0 as u8, (values & 0x00FF) as u8, (values >> 8) as u8];
        i2c.write(self.address, &buffer)
    }

    /// Read the direction of the I/O pins.  Bits set to 1 are configured as input pins with
    /// high-impedance output drivers.  Bits set to 0 are set as output pins.
    pub fn read_config(&self, i2c: &mut T) -> Result<u16, E> {
        let mut buffer = [0u8;2];
        i2c.write_read(self.address, &[Register::CONFIG_PORT0 as u8], &mut buffer)?;
        Ok(u16::from_le_bytes(buffer))
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub enum Address {
    ADDR_0x20 = 0x20,
    ADDR_0x21 = 0x21,
    ADDR_0x22 = 0x22,
    ADDR_0x23 = 0x23,
    ADDR_0x24 = 0x24,
    ADDR_0x25 = 0x25,
    ADDR_0x26 = 0x26,
    ADDR_0x27 = 0x27,
}

impl TryFrom<u8> for Address {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x20 => Ok(Address::ADDR_0x20),
            0x21 => Ok(Address::ADDR_0x21),
            0x22 => Ok(Address::ADDR_0x22),
            0x23 => Ok(Address::ADDR_0x23),
            0x24 => Ok(Address::ADDR_0x24),
            0x25 => Ok(Address::ADDR_0x25),
            0x26 => Ok(Address::ADDR_0x26),
            0x27 => Ok(Address::ADDR_0x27),
            _ => Err(()),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub enum Register {
    INPUT_PORT0 = 0x00,
    INPUT_PORT1 = 0x01,
    OUTPUT_PORT0 = 0x02,
    OUTPUT_PORT1 = 0x03,
    POLARITY_INVERT0 = 0x04,
    POLARITY_INVERT1 = 0x05,
    CONFIG_PORT0 = 0x06,
    CONFIG_PORT1 = 0x07,
}

#[cfg(test)]
mod tests {
    use embedded_hal_mock::i2c::{
        Mock,
        Transaction,
    };
    use super::*;

    #[test]
    fn test_read_inputs() {
        let addr = Address::ADDR_0x24;
        let expected_value = 0x1234u16;
        let expected = [
            Transaction::write_read(addr as u8, vec![Register::INPUT_PORT0 as u8], expected_value.to_le_bytes().to_vec())
        ];

        let mut i2c = Mock::new(&expected);
        let device = Tca9535::new(&i2c, addr).unwrap();
        let result = device.read_inputs(&mut i2c).unwrap();
        assert_eq!(result, expected_value);
    }

    #[test]
    fn test_read_outputs() {
        let addr = Address::ADDR_0x22;
        let expected_value = 0xBEEFu16;
        let expected = [
            Transaction::write_read(addr as u8, vec![Register::OUTPUT_PORT0 as u8], expected_value.to_le_bytes().to_vec())
        ];

        let mut i2c = Mock::new(&expected);
        let device = Tca9535::new(&i2c, addr).unwrap();
        let result = device.read_outputs(&mut i2c).unwrap();
        assert_eq!(result, expected_value);
    }
}
