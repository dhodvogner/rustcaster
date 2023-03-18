use crate::MAP_INSTANCE;
use crate::color;
use crate::draw::{draw_rect};

#[derive(Debug)]
pub struct Map {
    pub x: i32,
    pub y: i32,
    pub s: i32,
    pub data: Vec<i32>,
}

impl Map {
    pub fn global() -> &'static Map {
        MAP_INSTANCE.get().expect("Player instance not initialized")
    }

    pub fn new() -> Map {
        let x = 8;
        let y = 8;
        let s = 64;

        let data = [
            1,1,1,1,1,1,1,1,
            1,0,1,0,0,0,0,1,
            1,0,1,0,0,0,0,1,
            1,0,1,0,0,0,0,1,
            1,0,0,0,0,0,0,1,
            1,0,0,0,0,1,0,1,
            1,0,0,0,0,0,0,1,
            1,1,1,1,1,1,1,1,
        ];


        Map { 
            x, 
            y, 
            s, 
            data: data.to_vec()
         }
    }

    pub fn draw_2d(&self) {
        for y in 0..self.y  {
            for x in 0..self.x {
                let color;
                let index = y* self.x + x;
                if self.data[index as usize] == 1 {
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
}