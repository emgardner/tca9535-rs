/// Valid addresses for the TCA9535
#[allow(non_camel_case_types)]
#[repr(u8)]
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
pub enum Registers {
    INPUT_PORT0 = 0x00,
    INPUT_PORT1 = 0x01,
    OUTPUT_PORT0 = 0x02,
    OUTPUT_PORT1 = 0x03,
    POLARITY_INVERT0 = 0x04,
    POLARITY_INVERT1 = 0x05,
    CONFIG_PORT0 = 0x06,
    CONFIG_PORT1 = 0x07,
}
