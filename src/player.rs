use crate::PLAYER_INSTANCE;
use crate::color;
use crate::draw::draw_line;
use crate::draw::{draw_rect};
use crate::screen::Screen;
use crate::math::{fix_ang, deg_to_rad, sin, cos};

#[derive(Debug)]
pub struct Player {
    pub x: f64,
    pub y: f64,
    pub dx: f64,
    pub dy: f64,
    pub angle: f64,
    size: f64,

    speed: f64,
    pub turn_speed: f64,
}

impl Player {
    pub fn global() -> &'static mut Player {
        unsafe {
            PLAYER_INSTANCE.get_mut().expect("Player instance not initialized")
        }
    }

    pub fn new(x: f64, y: f64) -> Player {
        let angle = 90.0;
        let dx = cos(deg_to_rad(angle));
        let dy = -sin(deg_to_rad(angle));

        Player { 
            x, 
            y,
            dx,
            dy,
            angle,
            size: 8.0,
            speed: 8.0,
            turn_speed: 5.0,
        }
    }

    pub fn set_position(&mut self, x: f64, y: f64) {
        let mut px = x;
        let mut py = y;

        let half_size = self.size / 2.0;
        let width = Screen::global().width as f64;
        let height = Screen::global().height as f64;

        if px < 0.0 {
            px = 0.0;
        }

        if py < 0.0 {
            py = 0.0;
        }

        if px > width - half_size {
            px = width - half_size;
        }

        if py > height - half_size {
            py = height - half_size;
        }
        
        self.x = px;
        self.y = py;
    }

    pub fn draw_2d(&self) {
        let color = color::Color::new(255, 255, 0, 255);
        let half_size = self.size / 2.0;

        let psx = self.x - half_size;
        let psy = self.y - half_size;

        draw_rect(psx as i32, psy as i32, self.size as i32, self.size as i32, color);

        let x2 = self.x + self.dx * 20.0;
        let y2 = self.y + self.dy * 20.0;

        draw_line(self.x, self.y, x2, y2, color);
    }

    pub fn rotate(&mut self, angle: f64) {
        self.angle += angle;

        self.angle = fix_ang(self.angle);

        self.dx = cos(deg_to_rad(self.angle));
        self.dy = -sin(deg_to_rad(self.angle));
    }

    pub fn move_foward(&mut self) {
        let new_x = self.x + self.dx * self.speed;
        let new_y = self.y + self.dy * self.speed;
        self.set_position(new_x, new_y);
    }

    pub fn move_backward(&mut self) {
        let new_x = self.x - self.dx * self.speed;
        let new_y = self.y - self.dy * self.speed;
        self.set_position(new_x, new_y);
    }
}