pub struct Stack {
    data: Vec<u16>,
}

impl Stack {
    pub fn new() -> Stack {
        Stack { data: vec![] }
    }

    pub fn push(&mut self, data: u16) {
        self.data.push(data);
    }

    pub fn pop(&mut self) -> u16 {
        let value = self.data.pop().unwrap();

        value
    }
}
