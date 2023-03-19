use crate::color;
use crate::map::Map;
use crate::player::Player;
use crate::draw::draw_line;
use crate::math::{fix_ang, deg_to_rad, sin, cos, tan};
 
pub fn cast_ray() {
    let mut rx; 
    let mut ry;

    let mut xo = 0.0;
    let mut yo = 0.0;

    let mapx = Map::global().x;
    let mapy = Map::global().y;
    let maps = Map::global().s;

    let px = Player::global().x as f64;
    let py = Player::global().y as f64;
    let pa = Player::global().angle;
    let mut ra = fix_ang(pa + 30.0);

    for _r in 0..60 {
        // Check vertical lines
        let mut dof = 0;
        let n_tan = tan(deg_to_rad(ra));

        if cos(deg_to_rad(ra))> 0.001 { // looking left
            rx = ((px / 64.0).floor() * 64.0) + 64.0;
            ry = (px - rx) * n_tan + py;
            xo = 64.0;
            yo = -xo * n_tan;
        }
        else if cos(deg_to_rad(ra)) < -0.001 { // looking right
            rx = ((px / 64.0).floor() * 64.0) - 0.0001;
            ry = (px - rx) * n_tan + py;
            xo = -64.0;
            yo = -xo * n_tan;
        }
        else { // looking up or down
            rx = px;
            ry = py;
            dof = 8;
        }

        let mut dist_vertical = 100000.0;

        while dof < 8 {
            let mx = rx as i32 / 64;
            let my = ry as i32 / 64;
            let mp = my * mapx + mx;

            if mp > 0 && mp < mapx * mapy && Map::global().data[mp as usize] == 1 {
                dof = 8; // hit wall
                dist_vertical = cos(deg_to_rad(ra)) * (rx-px) - sin(deg_to_rad(ra)) * (ry-py);
            } else { // next line
                rx += xo;
                ry += yo;
                dof += 1;
            }
        }

        let vx = rx;
        let vy = ry;

        // Check horizonal lines
        let mut dof = 0;
        let mut dist_horizontal = 100000.0;
        let a_tan = 1.0 / tan(deg_to_rad(ra));

        if sin(deg_to_rad(ra)) > 0.001 { // looking up
            ry = ((py / 64.0).floor() * 64.0) - 0.0001;
            rx = (py - ry) * a_tan + px;
            yo = -64.0;
            xo = -yo * a_tan;
        }
        else if sin(deg_to_rad(ra)) < -0.001 { // looking down
            ry = ((py / 64.0).floor() * 64.0) + 64.0;
            rx = (py - ry) * a_tan + px;
            yo = 64.0;
            xo = -yo * a_tan;
        }
        else { // looking left or right
            rx = px;
            ry = py;
            dof = 8;   
        }

        while dof < 8 {
            let mx = rx as i32 / 64;
            let my = ry as i32 / 64;
            let mp = my * mapx + mx;

            if mp > 0 && mp < mapx * mapy && Map::global().data[mp as usize] == 1 {
                dof = 8; // hit wall
                dist_horizontal = cos(deg_to_rad(ra)) * (rx-px) - sin(deg_to_rad(ra)) * (ry-py);
            } else { // next line
                rx += xo;
                ry += yo;
                dof += 1;
            }
        }

        if dist_vertical < dist_horizontal {
            rx = vx;
            ry = vy;
            dist_horizontal = dist_vertical;
        }

        let color = color::Color::new(0, 255, 0, 255);
        draw_line(px, py, rx, ry, color );

        //fix fisheye 
        let ca= fix_ang(pa - ra); 
        dist_horizontal = dist_horizontal * cos(deg_to_rad(ca));

        // Draw the 3D walls
        // Render window is 320x160
        let mut line_height = (maps as f64 * 320.0) / dist_horizontal;
        if line_height > 320.0 { line_height = 320.0; }
        let line_off = 160.0 - (line_height / 2.0);

        let offset = 530.0;
        let line_width = 8;
        let x = _r as f64 * line_width as f64 + offset;

        // Simulate line width
        let color = color::Color::new(255, 255, 255, 255);
        for i in 0..line_width {
            draw_line(x + i as f64, line_off, x + i as f64, line_off + line_height, color);
        }

        ra = fix_ang(ra - 1.0);
    }
}

