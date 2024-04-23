pub struct Memory {
    pub data: [u8; 4096],
}

impl Memory {
    pub fn get_as_u16(&self, location: u16) -> u16 {
        u16::from(self.data[usize::from(location)]) << 8
            | u16::from(self.data[usize::from(location + 1)])
    }

    pub fn get_as_u8(&self, location: u16) -> u8 {
        self.data[usize::from(location)]
    }

    pub fn set(&mut self, location: u16, data: u8) {
        self.data[usize::from(location)] = data;
    }
}
