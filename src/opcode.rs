pub struct Opcode {
    pub category: u8,
    pub x: usize,
    pub y: usize,
    pub n: u8,
}

impl Opcode {
    pub fn kk(&self) -> u8 {
        (self.y as u8) << 4 | self.n
    }

    pub fn nnn(&self) -> u16 {
        (self.x << 8) as u16 | (self.y << 4) as u16 | u16::from(self.n)
    }
}

impl From<u16> for Opcode {
    fn from(value: u16) -> Self {
        let nibble_1 = ((value & 0xF000) >> 12) as u8;
        let nibble_2 = ((value & 0x0F00) >> 8) as u8;
        let nibble_3 = ((value & 0x00F0) >> 4) as u8;
        let nibble_4 = (value & 0x000F) as u8;

        Opcode {
            category: nibble_1,
            x: nibble_2 as usize,
            y: nibble_3 as usize,
            n: nibble_4,
        }
    }
}
