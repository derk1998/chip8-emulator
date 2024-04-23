pub struct ProgramCounter {
    counter: u16,
}

impl ProgramCounter {
    pub fn new() -> ProgramCounter {
        ProgramCounter { counter: 0x200 }
    }

    pub fn increment(&mut self) {
        self.counter += 2
    }

    pub fn decrement(&mut self) {
        self.counter -= 2
    }

    pub fn set(&mut self, new_counter_value: u16) {
        self.counter = new_counter_value;
    }

    pub fn get(&self) -> u16 {
        return self.counter;
    }
}
