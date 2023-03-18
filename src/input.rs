use crate::INPUT_INSTANCE;

#[derive(Debug)]
pub struct Input {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

impl Input {
    pub fn global() -> &'static mut Input {
        unsafe {
            INPUT_INSTANCE.get_mut().expect("Input instance not initialized")
        }
    }

    pub fn new() -> Input {
        Input {
            up: false,
            down: false,
            left: false,
            right: false,
        }
    }

    pub fn key_down(&mut self, key: u8) {
        match key {
            87 => self.up = true, // W
            83 => self.down = true, // S
            65 => self.right = true, // D
            68 => self.left = true, // A
            _ => {}
        }
    }

    pub fn key_up(&mut self, key: u8) {
        match key {
            87 => self.up = false, // W
            83 => self.down = false, // S
            65 => self.right = false, // D
            68 => self.left = false, // A
            _ => {}
        }
    }
}

