use std::sync::Mutex;

use crate::SCREEN_INSTANCE;

#[derive(Debug)]
pub struct Screen {
    pub width: i32,
    pub height: i32,
    pub output_buffer: Mutex<Vec<u8>>,
}

impl Screen {
    pub fn global() -> &'static Screen {
        SCREEN_INSTANCE.get().expect("Screen instance not initialized")
    }

    pub fn new(width: i32, height: i32) -> Screen {
        let buffer_size = width * height * 4;
        let output_buffer: Mutex<Vec<u8>> = Mutex::new(vec![0; buffer_size as usize]);

        Screen {
            width,
            height,
            output_buffer,
        }
    }
}