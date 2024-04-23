use crate::{
    display::Display,
    keypad::{Key, Keypad},
    memory::Memory,
    opcode::Opcode,
    program_counter::ProgramCounter,
    stack::Stack,
    timer::Timer,
};

pub struct Chip8<'a> {
    program_counter: ProgramCounter,
    memory: Memory,
    registers: [u8; 16],
    index_register: u16,
    stack: Stack,
    display: &'a mut Display,
    delay_timer: Timer,
    sound_timer: Timer,
    key_pad: Keypad,
}

impl<'a> Chip8<'a> {
    pub fn new(display: &mut Display, memory: Memory) -> Chip8 {
        Chip8 {
            program_counter: ProgramCounter::new(),
            memory,
            registers: [0; 16],
            index_register: 0,
            stack: Stack::new(),
            display,
            delay_timer: Timer::new(),
            sound_timer: Timer::new(),
            key_pad: Keypad::new(),
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
        self.registers[0xF] = 0;
        self.registers[opcode.x] |= self.registers[opcode.y];
    }

    fn op_8xy2(&mut self, opcode: &Opcode) {
        self.registers[0xF] = 0;
        self.registers[opcode.x] &= self.registers[opcode.y];
    }

    fn op_8xy3(&mut self, opcode: &Opcode) {
        self.registers[0xF] = 0;
        self.registers[opcode.x] ^= self.registers[opcode.y];
    }

    fn op_8xy4(&mut self, opcode: &Opcode) {
        let sum = self.registers[opcode.y] as u16 + self.registers[opcode.x] as u16;
        self.registers[opcode.x] = (sum & 0x00FF) as u8;
        self.registers[0xF] = if sum > 0xFF { 1 } else { 0 };
    }

    fn op_8xy5(&mut self, opcode: &Opcode) {
        let res = self.registers[opcode.x].overflowing_sub(self.registers[opcode.y]);
        self.registers[opcode.x] = res.0;
        self.registers[0xF] = if res.1 { 0 } else { 1 };
    }

    fn op_8xy6(&mut self, opcode: &Opcode) {
        self.registers[opcode.x] = self.registers[opcode.y];

        let lsb = self.registers[opcode.x] & 0x1;
        self.registers[opcode.x] >>= 1;
        self.registers[0xF] = lsb;
    }

    fn op_8xy7(&mut self, opcode: &Opcode) {
        let res = self.registers[opcode.y].overflowing_sub(self.registers[opcode.x]);
        self.registers[opcode.x] = res.0;
        self.registers[0xF] = if res.1 { 0 } else { 1 };
    }

    fn op_8xye(&mut self, opcode: &Opcode) {
        self.registers[opcode.x] = self.registers[opcode.y];

        let msb = (self.registers[opcode.x] & 0x80) >> 7;
        self.registers[opcode.x] <<= 1;
        self.registers[0xF] = msb;
    }

    fn op_9xy0(&mut self, opcode: &Opcode) {
        if self.registers[opcode.x] != self.registers[opcode.y] {
            self.program_counter.increment();
        }
    }

    fn op_annn(&mut self, opcode: &Opcode) {
        self.index_register = opcode.nnn();
    }

    fn op_bnnn(&mut self, opcode: &Opcode) {
        self.program_counter
            .set(opcode.nnn() + self.registers[0] as u16);
    }

    fn op_cxkk(&mut self, opcode: &Opcode) {
        self.registers[opcode.x] = rand::random::<u8>() & opcode.kk();
    }

    fn op_dxyn(&mut self, opcode: &Opcode) {
        let x = self.registers[opcode.x] as u16 % self.display.width();
        let y = self.registers[opcode.y] as u16 % self.display.height();
        self.registers[0xF] = 0;

        for row in 0..opcode.n {
            if y + row as u16 >= self.display.height() {
                break;
            }

            let sprite_byte = self.memory.get_as_u8(self.index_register + row as u16);

            for col in 0..8 {
                let pixel = (sprite_byte & (0x80 >> col)) >> 7 - col;

                if x + col as u16 >= self.display.width() {
                    break;
                }

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

    fn op_ex9e(&mut self, opcode: &Opcode) {
        if self.key_pad.is_key_down(self.registers[opcode.x]) {
            self.program_counter.increment();
        }
    }

    fn op_exa1(&mut self, opcode: &Opcode) {
        if !self.key_pad.is_key_down(self.registers[opcode.x]) {
            self.program_counter.increment();
        }
    }

    fn op_fx07(&mut self, opcode: &Opcode) {
        self.registers[opcode.x] = self.delay_timer.get();
    }

    fn op_fx0a(&mut self, opcode: &Opcode) {
        let res = self.key_pad.get_any_key_up();
        if res.is_some() {
            self.registers[opcode.x] = res.unwrap();
        } else {
            self.program_counter.decrement();
        }
    }

    fn op_fx15(&mut self, opcode: &Opcode) {
        self.delay_timer.set(self.registers[opcode.x]);
    }

    fn op_fx18(&mut self, opcode: &Opcode) {
        self.sound_timer.set(self.registers[opcode.x]);
    }

    fn op_fx1e(&mut self, opcode: &Opcode) {
        self.index_register += self.registers[opcode.x] as u16;
    }

    fn op_fx29(&mut self, opcode: &Opcode) {
        self.index_register = (self.registers[opcode.x] * 5 + 0x50) as u16;
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
                .set(self.index_register as u16, self.registers[i]);
            self.index_register += 1;
        }
    }

    fn op_fx65(&mut self, opcode: &Opcode) {
        for i in 0..=opcode.x {
            self.registers[i] = self.memory.get_as_u8(self.index_register as u16);
            self.index_register += 1;
        }
    }

    pub fn handle_key_up(&mut self, key: Key) {
        self.key_pad.key_up(key);
    }

    pub fn handle_key_down(&mut self, key: Key) {
        self.key_pad.key_down(key);
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
                n: 0x7,
                ..
            } => self.op_8xy7(&opcode),
            Opcode {
                category: 0x8,
                n: 0xE,
                ..
            } => self.op_8xye(&opcode),
            Opcode { category: 0x9, .. } => self.op_9xy0(&opcode),
            Opcode { category: 0xA, .. } => self.op_annn(&opcode),
            Opcode { category: 0xB, .. } => self.op_bnnn(&opcode),
            Opcode { category: 0xC, .. } => self.op_cxkk(&opcode),
            Opcode { category: 0xD, .. } => self.op_dxyn(&opcode),
            Opcode {
                category: 0xE,
                y: 0x9,
                n: 0xE,
                ..
            } => self.op_ex9e(&opcode),
            Opcode {
                category: 0xE,
                y: 0xA,
                n: 0x1,
                ..
            } => self.op_exa1(&opcode),
            Opcode {
                category: 0xF,
                y: 0x0,
                n: 0x7,
                ..
            } => self.op_fx07(&opcode),
            Opcode {
                category: 0xF,
                y: 0x0,
                n: 0xA,
                ..
            } => self.op_fx0a(&opcode),
            Opcode {
                category: 0xF,
                y: 0x1,
                n: 0x5,
                ..
            } => self.op_fx15(&opcode),
            Opcode {
                category: 0xF,
                y: 0x1,
                n: 0x8,
                ..
            } => self.op_fx18(&opcode),
            Opcode {
                category: 0xF,
                y: 0x1,
                n: 0xE,
                ..
            } => self.op_fx1e(&opcode),
            Opcode {
                category: 0xF,
                y: 0x2,
                n: 0x9,
                ..
            } => self.op_fx29(&opcode),
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

        self.delay_timer.tick();
        self.sound_timer.tick();
    }

    fn fetch(&mut self) -> Opcode {
        let program_counter = self.program_counter.get();
        let opcode = self.memory.get_as_u16(program_counter);
        self.program_counter.increment();

        opcode.into()
    }
}
