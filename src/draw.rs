#[cfg(target_arch = "wasm32")]
use web_sys::console;

use std::sync::MutexGuard;

use crate::color;
use crate::Screen;

pub fn put_pixel(x: f64, y: f64, color: color::Color, output_buffer: &mut MutexGuard<Vec<u8>>) {
    let x = x as i32;
    let y = y as i32;

    // Prevent drawing outside of the screen
    if x < 0 || y < 0 { return; }
    if x >= Screen::global().width || y >= Screen::global().height { return;}

    let number = y * Screen::global().width + x;
    let rgba_index: usize = (number * 4) as usize;

    output_buffer[rgba_index + 0] = color.r;
    output_buffer[rgba_index + 1] = color.g;
    output_buffer[rgba_index + 2] = color.b;
    output_buffer[rgba_index + 3] = color.a;
}

pub fn draw_pixel(x: f64, y: f64, color: color::Color) {
    let mut locked_buffer = Screen::global().output_buffer.lock().unwrap();
    put_pixel(x, y, color, &mut locked_buffer);
}

pub fn fill(color: color::Color) {
    let mut locked_buffer = Screen::global().output_buffer.lock().unwrap();
    for y in 0..Screen::global().height {
        for x in 0..Screen::global().width {
            put_pixel(x as f64, y as f64, color, &mut locked_buffer);
        }
    }
}

pub fn draw_rect(x: i32, y: i32, width: i32, height: i32, color: color::Color) {
    let mut locked_buffer = Screen::global().output_buffer.lock().unwrap();
    for ny in y..y + width {
        for nx in x..x + height {
            put_pixel(nx as f64, ny as f64, color, &mut locked_buffer);
        }
    }
}

pub fn draw_line(x1: f64, y1: f64, x2: f64, y2: f64, color: color::Color) {
    let start_x = x1.round();
    let start_y = y1.round();

    let target_x = x2.round();
    let target_y = y2.round();

    let mut x = start_x;
    let mut y = start_y;
    let mut dx = (target_x - start_x).round();
    let mut dy = (target_y - start_y).round();

    let mut x_inc = 1.0;
    let mut y_inc = 1.0;

    if dx < 0.0 {
        dx = -dx;
        x_inc = -1.0;
    }

    if dy < 0.0 {
        dy = -dy;
        y_inc = -1.0;
    }

    let mut error = dx - dy;
    let mut loop_count = 0;

    let mut locked_buffer = Screen::global().output_buffer.lock().unwrap();

    loop {
        if loop_count > 1000 {
            cfg_if::cfg_if! {
                if #[cfg(target_arch = "wasm32")] {
                console::warn_1(&format!("Warning: Loop count exceeded").into());
                } else {
                    println!("Warning: Loop count exceeded");
                } 
            }
            break;
        }

        put_pixel(x, y, color, &mut locked_buffer);

        if x == target_x && y == target_y {
            break;
        }

        if x < 0.0 || y < 0.0 {
            // console::log_1(&format!("Warning: x < 0.0 || y < 0.0").into());
            break;
        }

        if x > Screen::global().width as f64 || y > Screen::global().height as f64 {
            // console::log_1(&format!("Warning: x > Screen::global().width || y > Screen::global().height").into());
            break;
        }

        let error2 = error * 2.0;

        if error2 > -dy {
            error -= dy;
            x = x + x_inc;
        }

        if error2 < dx {
            error += dx;
            y = y + y_inc;
        }

        loop_count += 1;
    }
}

// pub fn draw_circle(x: usize, y: usize, radius: f64, color: color::Color) {
//     let min_angle = acos(1.0 - 1.0/radius as f64);
//     let mut angle: f64 = 0.0;

//     while  angle < 360.0 {
//         let mut x1 = radius * cos(angle as f64 * PI / 180.0);
//         let mut y1 = radius * sin(angle as f64 * PI / 180.0);
//         put_pixel(x + x1 as usize, y + y1 as usize, color);
//         angle += 0.1;
//     }
// }