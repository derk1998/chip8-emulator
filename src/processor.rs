use crate::display::{self, Display};

pub struct ProgramCounter {
    counter: u16,
}

impl ProgramCounter {
    fn increment(&mut self) {
        self.counter += 2
    }

    fn set(&mut self, new_counter_value: u16) {
        self.counter = new_counter_value;
    }

    fn get(&self) -> u16 {
        return self.counter;
    }
}

pub struct Stack {
    data: Vec<u16>,
}

impl Stack {
    fn push(&mut self, data: u16) {
        self.data.push(data);
    }

    fn pop(&mut self) -> u16 {
        let value = self.data.last().copied().unwrap();
        self.data.dedup();

        value
    }
}

pub struct Memory {
    pub data: [u8; 4096],
}

impl Memory {
    fn get_as_u16(&self, location: u16) -> u16 {
        u16::from(self.data[usize::from(location)]) << 8
            | u16::from(self.data[usize::from(location + 1)])
    }

    fn get_as_u8(&self, location: u16) -> u8 {
        self.data[usize::from(location)]
    }

    fn set(&mut self, location: u16, data: u8) {
        self.data[usize::from(location)] = data;
    }
}

pub struct Opcode {
    category: u8,
    x: usize,
    y: usize,
    n: u8,
}

impl Opcode {
    fn kk(&self) -> u8 {
        (self.y as u8) << 4 | self.n
    }

    fn nnn(&self) -> u16 {
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

pub struct Chip8<'a> {
    program_counter: ProgramCounter,
    memory: Memory,
    registers: [u8; 16],
    index_register: u16,
    stack: Stack,
    display: &'a mut Display<'a>,
}

impl<'a> Chip8<'a> {
    pub fn new(display: &'a mut Display<'a>, memory: Memory) -> Chip8<'a> {
        Chip8 {
            program_counter: ProgramCounter { counter: 0x200 },
            memory,
            registers: [0; 16],
            index_register: 0,
            stack: Stack { data: vec![] },
            display,
        }
    }

    fn op_00e0(&mut self) {
        self.display.clear();
    }

    fn op_00ee(&mut self) {
        self.program_counter.set(self.stack.pop());
    }

    fn op_1nnn(&mut self, opcode: &Opcode) {
        self.program_counter.set(opcode.nnn());
    }

    fn op_2nnn(&mut self, opcode: &Opcode) {
        self.stack.push(self.program_counter.get());
        self.program_counter.set(opcode.nnn());
    }

    fn op_3xkk(&mut self, opcode: &Opcode) {
        if self.registers[opcode.x] == opcode.kk() {
            self.program_counter.increment();
        }
    }

    fn op_4xkk(&mut self, opcode: &Opcode) {
        if self.registers[opcode.x] != opcode.kk() {
            self.program_counter.increment();
        }
    }

    fn op_5xy0(&mut self, opcode: &Opcode) {
        if self.registers[opcode.x] == self.registers[opcode.y] {
            self.program_counter.increment();
        }
    }

    fn op_6xkk(&mut self, opcode: &Opcode) {
        self.registers[opcode.x] = opcode.kk();
    }

    fn op_7xkk(&mut self, opcode: &Opcode) {
        self.registers[opcode.x] = self.registers[opcode.x].overflowing_add(opcode.kk()).0;
    }

    fn op_8xy0(&mut self, opcode: &Opcode) {
        self.registers[opcode.x] = self.registers[opcode.y];
    }

    fn op_8xy1(&mut self, opcode: &Opcode) {
        self.registers[opcode.x] |= self.registers[opcode.y];
    }

    fn op_8xy2(&mut self, opcode: &Opcode) {
        self.registers[opcode.x] &= self.registers[opcode.y];
    }

    fn op_8xy3(&mut self, opcode: &Opcode) {
        self.registers[opcode.x] ^= self.registers[opcode.y];
    }

    fn op_8xy4(&mut self, opcode: &Opcode) {
        let sum = self.registers[opcode.y] as u16 + self.registers[opcode.x] as u16;
        self.registers[0xF] = if sum > 0xFF { 1 } else { 0 };
        self.registers[opcode.x] = (sum & 0xFF) as u8;
    }

    fn op_8xy5(&mut self, opcode: &Opcode) {
        let res = self.registers[opcode.x].overflowing_sub(self.registers[opcode.y]);
        self.registers[0xF] = if res.1 { 1 } else { 0 };
        self.registers[opcode.x] = res.0;
    }

    fn op_8xy6(&mut self, opcode: &Opcode) {
        self.registers[0xF] = self.registers[opcode.x] & 0x1;
        self.registers[opcode.x] >>= 1;
    }

    fn op_8xye(&mut self, opcode: &Opcode) {
        self.registers[0xF] = (self.registers[opcode.x] & 0x80) >> 7;
        self.registers[opcode.x] <<= 1;
    }

    fn op_9xy0(&mut self, opcode: &Opcode) {
        if self.registers[opcode.x] != self.registers[opcode.y] {
            self.program_counter.increment();
        }
    }

    fn op_annn(&mut self, opcode: &Opcode) {
        self.index_register = opcode.nnn();
    }

    fn op_dxyn(&mut self, opcode: &Opcode) {
        let x = self.registers[opcode.x] as u16 % self.display.width();
        let y = self.registers[opcode.y] as u16 % self.display.height();
        self.registers[0xF] = 0;

        for row in 0..opcode.n {
            let sprite_byte = self.memory.get_as_u8(self.index_register + row as u16);

            for col in 0..8 {
                let pixel = (sprite_byte & (0x80 >> col)) >> 7 - col;

                if pixel == 1 {
                    if !self
                        .display
                        .flip_pixel((x + col) as usize, (y + row as u16) as usize)
                    {
                        self.registers[0xF] = 1;
                    }
                }
            }
        }
        self.display.display();
    }

    fn op_fx33(&mut self, opcode: &Opcode) {
        let mut value = self.registers[opcode.x];

        for i in (0..=2).rev() {
            self.memory.set(self.index_register + i, value % 10);
            if i > 0 {
                value /= 10;
            }
        }
    }

    fn op_fx55(&mut self, opcode: &Opcode) {
        for i in 0..=opcode.x {
            self.memory
                .set(self.index_register + i as u16, self.registers[i]);
        }
    }

    fn op_fx65(&mut self, opcode: &Opcode) {
        for i in 0..=opcode.x {
            self.registers[i] = self.memory.get_as_u8(self.index_register + i as u16);
        }
    }

    pub fn emulate_cycle(&mut self) {
        let opcode = self.fetch();

        match opcode {
            Opcode {
                category: 0x0,
                x: 0x0,
                y: 0xe,
                n: 0x0,
            } => self.op_00e0(),
            Opcode {
                category: 0x0,
                x: 0x0,
                y: 0xe,
                n: 0xe,
            } => self.op_00ee(),
            Opcode { category: 0x1, .. } => self.op_1nnn(&opcode),
            Opcode { category: 0x2, .. } => self.op_2nnn(&opcode),
            Opcode { category: 0x3, .. } => self.op_3xkk(&opcode),
            Opcode { category: 0x4, .. } => self.op_4xkk(&opcode),
            Opcode { category: 0x5, .. } => self.op_5xy0(&opcode),
            Opcode { category: 0x6, .. } => self.op_6xkk(&opcode),
            Opcode { category: 0x7, .. } => self.op_7xkk(&opcode),
            Opcode {
                category: 0x8,
                n: 0x0,
                ..
            } => self.op_8xy0(&opcode),
            Opcode {
                category: 0x8,
                n: 0x1,
                ..
            } => self.op_8xy1(&opcode),
            Opcode {
                category: 0x8,
                n: 0x2,
                ..
            } => self.op_8xy2(&opcode),
            Opcode {
                category: 0x8,
                n: 0x3,
                ..
            } => self.op_8xy3(&opcode),
            Opcode {
                category: 0x8,
                n: 0x4,
                ..
            } => self.op_8xy4(&opcode),
            Opcode {
                category: 0x8,
                n: 0x5,
                ..
            } => self.op_8xy5(&opcode),
            Opcode {
                category: 0x8,
                n: 0x6,
                ..
            } => self.op_8xy6(&opcode),
            Opcode {
                category: 0x8,
                n: 0xE,
                ..
            } => self.op_8xye(&opcode),
            Opcode { category: 0x9, .. } => self.op_9xy0(&opcode),
            Opcode { category: 0xA, .. } => self.op_annn(&opcode),
            Opcode { category: 0xD, .. } => self.op_dxyn(&opcode),
            Opcode {
                category: 0xF,
                y: 0x3,
                n: 0x3,
                ..
            } => self.op_fx33(&opcode),
            Opcode {
                category: 0xF,
                y: 0x5,
                n: 0x5,
                ..
            } => self.op_fx55(&opcode),
            Opcode {
                category: 0xF,
                y: 0x6,
                n: 0x5,
                ..
            } => self.op_fx65(&opcode),
            _ => {
                println!("Unsupported operation: {}", opcode.category);
            }
        }

        //update timers
    }

    fn fetch(&mut self) -> Opcode {
        let program_counter = self.program_counter.get();
        let opcode = self.memory.get_as_u16(program_counter);
        self.program_counter.increment();

        opcode.into()
    }
}
