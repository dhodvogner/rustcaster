use crate::INPUT_INSTANCE;

#[derive(Debug)]
pub struct Input {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub strafe_left: bool,
    pub strafe_right: bool,
    pub open_door: bool,
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
            strafe_left: false,
            strafe_right: false,
            open_door: false,
        }
    }

    pub fn key_down(&mut self, key: u8) {
        match key {
            87 => self.up = true, // W
            83 => self.down = true, // S
            65 => self.right = true, // A
            68 => self.left = true, // D
            81 => self.strafe_left = true, // Q
            69 => self.strafe_right = true, // E
            32 => self.open_door = true, // Space
            _ => {}
        }
    }

    pub fn key_up(&mut self, key: u8) {
        match key {
            87 => self.up = false, // W
            83 => self.down = false, // S
            65 => self.right = false, // D
            68 => self.left = false, // A
            81 => self.strafe_left = false, // Q
            69 => self.strafe_right = false, // E
            32 => self.open_door = false, // Space
            _ => {}
        }
    }
}

