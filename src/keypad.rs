#[derive(Clone)]
pub enum Key {
    Key0 = 0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    KeyA,
    KeyB,
    KeyC,
    KeyD,
    KeyE,
    KeyF,
}

pub struct Keypad {
    keys: [bool; 16],
    save_key: Option<u8>,
}

impl Keypad {
    pub fn new() -> Self {
        Keypad {
            keys: [false; 16],
            save_key: None,
        }
    }

    pub fn key_down(&mut self, key: Key) {
        self.keys[key as usize] = true;
    }

    pub fn key_up(&mut self, key: Key) {
        self.keys[key as usize] = false;
    }

    pub fn is_key_down(&self, key: u8) -> bool {
        self.keys[key as usize]
    }

    pub fn get_any_key_up(&mut self) -> Option<u8> {
        let mut key: Option<u8> = None;
        if self.save_key.is_none() {
            self.save_key = self.get_any_key_down();
        } else {
            if !self.is_key_down(self.save_key.unwrap() as u8) {
                key = self.save_key;
                self.save_key = None;
            }
        }
        key
    }

    pub fn get_any_key_down(&self) -> Option<u8> {
        for i in 0..self.keys.len() {
            if self.keys[i] {
                return Some(i as u8);
            }
        }

        None
    }
}
