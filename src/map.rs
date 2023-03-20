use crate::MAP_INSTANCE;
use crate::color;
use crate::draw::{draw_rect};

#[derive(Debug)]
pub struct Map {
    pub x: i32,
    pub y: i32,
    pub s: i32,
    pub data: Vec<i32>,
    pub floor_data: Vec<i32>,
    pub ceiling_data: Vec<i32>,
}

impl Map {
    pub fn global() -> &'static mut Map {
        unsafe {
            MAP_INSTANCE.get_mut().expect("Map instance not initialized")
        }
    }

    pub fn new() -> Map {
        let x = 8;
        let y = 8;
        let s = 64;

        let data = [
            1,1,1,1,1,3,1,1,
            1,0,0,1,0,0,0,1,
            1,0,0,4,0,2,0,1,
            1,1,4,1,0,0,0,1,
            2,0,0,0,0,0,0,1,
            2,0,0,0,0,1,0,1,
            2,0,0,0,0,0,0,1,
            1,1,3,1,3,1,3,1,
        ];

        let floor_data = [
            0,0,0,0,0,0,0,0,
            0,0,0,0,1,1,0,0,
            0,0,0,0,2,0,0,0,
            0,0,0,0,0,0,0,0,
            0,0,2,0,0,0,0,0,
            0,0,0,0,0,0,0,0,
            0,1,1,1,1,0,0,0,
            0,0,0,0,0,0,0,0,	
        ];

        let ceiling_data = [
            0,0,0,0,0,0,0,0,
            0,1,1,1,0,0,0,0,
            0,1,1,1,0,0,0,0,
            0,1,1,1,0,0,1,0,
            0,1,3,1,0,0,0,0,
            0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,
        ];


        Map { 
            x, 
            y, 
            s, 
            data: data.to_vec(),
            floor_data: floor_data.to_vec(),
            ceiling_data: ceiling_data.to_vec(),
         }
    }

    pub fn draw_2d(&self) {
        for y in 0..self.y  {
            for x in 0..self.x {
                let color;
                let index = y* self.x + x;
                if self.data[index as usize] > 0 {
                    color = color::Color::new(255, 255, 255, 255);
                } else {
                    color = color::Color::new(0, 0, 0, 255);
                }

                let xo = (x * self.s) + 1;
                let yo = (y * self.s) + 1;

                draw_rect(xo, yo, self.s - 2, self.s - 2, color);
            }
        }
    }

    pub fn get_wall(&self, x: i32, y: i32) -> i32 {
        let index = y * self.x + x;
        self.data[index as usize]
    }

    pub fn set_wall(&mut self, index: usize, value: i32) {
        self.data[index as usize] = value;
    }

}