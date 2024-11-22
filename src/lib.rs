#![no_std]
pub mod registers;

use crate::registers::Registers;
use embedded_hal_async::i2c::I2c as AsyncI2c;

bitflags::bitflags! {
    #[derive(Debug, Copy, Clone)]
    pub struct Port: u16 {
        const P00 = 0b0000_0000_0000_0001;
        const P01 = 0b0000_0000_0000_0010;
        const P02 = 0b0000_0000_0000_0100;
        const P03 = 0b0000_0000_0000_1000;
        const P04 = 0b0000_0000_0001_0000;
        const P05 = 0b0000_0000_0010_0000;
        const P06 = 0b0000_0000_0100_0000;
        const P07 = 0b0000_0000_1000_0000;

        const P10 = 0b0000_0001_0000_0000;
        const P11 = 0b0000_0010_0000_0000;
        const P12 = 0b0000_0100_0000_0000;
        const P13 = 0b0000_1000_0000_0000;
        const P14 = 0b0001_0000_0000_0000;
        const P15 = 0b0010_0000_0000_0000;
        const P16 = 0b0100_0000_0000_0000;
        const P17 = 0b1000_0000_0000_0000;
    }
}

pub enum Error<E> {
    I2CError(E),
}

pub struct TCA9535<I2C> {
    i2c: I2C,
    address: u8,
}

impl<I2C, E> TCA9535<I2C>
where
    I2C: AsyncI2c<Error = E>,
{
    pub fn new(i2c: I2C, address: u8) -> TCA9535<I2C> {
        Self { i2c, address }
    }

    pub async fn write_port(&mut self, register: Registers, port: Port) -> Result<(), E> {
        let bytes = port.bits().to_le_bytes();
        let buffer = [register as u8, bytes[0], bytes[1]];
        self.i2c.write(self.address, &buffer).await
    }

    pub async fn read_port(&mut self, register: Registers) -> Result<Port, E> {
        let mut buffer = [0u8; 2];
        self.i2c
            .write_read(self.address as u8, &[register as u8], &mut buffer)
            .await?;
        Ok(Port::from_bits(u16::from_le_bytes(buffer)).unwrap())
    }

    pub async fn clear_outputs(&mut self) -> Result<(), E> {
        self.i2c
            .write(self.address, &[Registers::OUTPUT_PORT0 as u8, 0, 0])
            .await
    }

    pub async fn write_config(&mut self, port: Port) -> Result<(), E> {
        let bytes = port.bits().to_le_bytes();
        let buffer = [Registers::CONFIG_PORT0 as u8, bytes[0], bytes[1]];
        self.i2c.write(self.address, &buffer).await
    }

    pub async fn write_outputs(&mut self, port: Port) -> Result<(), E> {
        self.write_port(Registers::OUTPUT_PORT0, port).await
    }
}
