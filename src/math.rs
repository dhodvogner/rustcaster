use crate::screen::Screen;

static PI: f64 = 3.14159265359;

pub fn deg_to_rad(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn fix_ang(a: f64) -> f64 {
    let mut res = a;
    if res > 359.0 {
        res -= 360.0;
    }
    if res < 0.0 {
        res += 360.0;
    }
    res
}

pub fn sin(a: f64) -> f64 {
    a.sin()
}

pub fn cos(a: f64) -> f64 {
    a.cos()
}

pub fn tan(a: f64) -> f64 {
    a.tan()
}

pub fn confine_point_to_screen(x: f64, y: f64, width: f64, height: f64) -> (f64, f64) {
    let screen_width = Screen::global().width as f64;
    let screen_height = Screen::global().height as f64;
    let mut x = x;
    let mut y = y;

    if x < 0.0 {
        x = 0.0;
    }
    if y < 0.0 {
        y = 0.0;
    }
    if x > screen_width - width {
        x = screen_width - width;
    }
    if y > screen_height - height {
        y = screen_height - height;
    }

    (x, y)
}
