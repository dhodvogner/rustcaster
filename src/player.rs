use crate::PLAYER_INSTANCE;
use crate::color;
use crate::draw::draw_line;
use crate::draw::{draw_rect};
use crate::math::{fix_ang, deg_to_rad, sin, cos, confine_point_to_screen};
use crate::map::Map;

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

    offset_x: f64,
    offset_y: f64,
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

        let (offset_x, offset_y) = Player::calculate_offset(dx, dy);

        Player { 
            x, 
            y,
            dx,
            dy,
            angle,
            size: 8.0,
            speed: 15.0,
            turn_speed: 20.0,
            offset_x,
            offset_y,
        }
    }

    pub fn calculate_offset(dx: f64, dy: f64) -> (f64, f64) {
        let mut offset_x = 20.0;
        let mut offset_y = 20.0;

        if dx < 0.0 { offset_x = -20.0; }
        if dy < 0.0 { offset_y = -20.0; }

        (offset_x, offset_y)
    }

    pub fn set_position(&mut self, x: f64, y: f64) {
        // Usign half size because the player origin should be in the center.
        let half_size = self.size / 2.0; 
        let (px, py) = confine_point_to_screen(x, y, half_size, half_size);

        self.x = px;
        self.y = py;
    }

    pub fn get_position(&self) -> (f64, f64) {
        (self.x.clone(), self.y.clone())
    }

    pub fn draw_2d(&self) {
        let color = color::Color::new(255, 255, 0, 255);

        // Usign half size because the player origin should be in the center.
        let half_size = self.size / 2.0;
        draw_rect(
            (self.x - half_size) as i32, 
            (self.y - half_size) as i32, 
            self.size as i32, 
            self.size as i32, color
        );

        // Draw the player direction line
        let direction_end_x = self.x + self.dx * 20.0;
        let direction_end_y = self.y + self.dy * 20.0;

        draw_line(self.x, self.y, direction_end_x, direction_end_y, color);
    }

    pub fn rotate(&mut self, angle: f64, delta_time: f64) {
        self.angle += angle * delta_time;

        self.angle = fix_ang(self.angle);

        self.dx = cos(deg_to_rad(self.angle));
        self.dy = -sin(deg_to_rad(self.angle));

        // Recalculate the offset
        let (offset_x, offset_y) = Player::calculate_offset(self.dx, self.dy);
        self.offset_x = offset_x;
        self.offset_y = offset_y;
    }

    pub fn translate(&mut self, dir_x: i8, dir_y: i8, delta_time: f64) {
        let mut new_x = self.x;
        let mut new_y = self.y;

        let speed = self.speed * delta_time;
        let dx = self.dx * speed;
        let dy = self.dy * speed;

        // For collision detection
        let ipx = (self.x / 64.0) as i32; // Player x position in the map
        let ipx_add_xoff = ((self.x + self.offset_x) / 64.0) as i32; // Tile in front of the player
        let ipx_sub_xoff = ((self.x - self.offset_x) / 64.0) as i32; // Tile behind the player
        let ipy = (self.y / 64.0) as i32; // Player y position in the map
        let ipy_add_yoff = ((self.y + self.offset_y) / 64.0) as i32; // Tile left of the player?
        let ipy_sub_yoff = ((self.y - self.offset_y) / 64.0) as i32; // Tile right of the player?

        let map = Map::global();

        if dir_x > 0 { // Move forward
            if map.get_wall(ipx_add_xoff, ipy,) == 0 { new_x += dx; }
            if map.get_wall( ipx, ipy_add_yoff) == 0 { new_y += dy; }
        }

        if dir_x < 0 { // Move backward
            if map.get_wall(ipx_sub_xoff, ipy) == 0 { new_x -= dx; }
            if map.get_wall(ipx, ipy_sub_yoff) == 0 { new_y -= dy; }
        }

        if dir_y > 0 { // Strafe right
            if map.get_wall(ipx_add_xoff, ipy) == 0 { new_x += dy; }
            if map.get_wall(ipx, ipy_sub_yoff) == 0 { new_y -= dx; }
        }

        if dir_y < 0 { // Strafe left
            if map.get_wall(ipx_sub_xoff, ipy) == 0 { new_x -= dy; }
            if map.get_wall(ipx, ipy_add_yoff) == 0 { new_y += dx; }
        }

        self.set_position(new_x, new_y);
    }
}