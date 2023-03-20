use crate::color;
use crate::map::Map;
use crate::player::Player;
use crate::draw::{draw_line, draw_pixel};
use crate::math::{fix_ang, deg_to_rad, sin, cos, tan};
use crate::texture::TEXTURE;
 
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

    let mut hmt = 0;
    let mut vmt = 0;

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
                vmt = Map::global().data[mp as usize] - 1;
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

        let mut shade = 1.0;

        if dist_vertical < dist_horizontal {
            hmt = vmt;
            shade = 0.5;
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

        let ty_step = 32.0 / line_height;
        let mut ty_off = 0.0;

        if line_height > 320.0 { 
            ty_off = (line_height - 320.0) / 2.0;
            line_height = 320.0; 
        }
        let line_off = 160.0 - (line_height / 2.0);

        let offset = 530.0;
        let line_width = 8;
        let x = _r as f64 * line_width as f64 + offset;

        let mut ty = ty_off * ty_step + (hmt * 32) as f64;
        let mut tx = 0.0;

        if shade == 1.0 {
            tx = rx / 2.0 % 32.0;
            if ra > 180.0 { tx = 31.0 - tx; }
        } else {
            tx = ry / 2.0 % 32.0;
            if ra > 90.0 && ra < 270.0 { tx = 31.0 - tx; }
        }

        // Drawing the walls
        for y in 0..line_height as i32 {

            let mut texture_index = (ty.round() * 32.0 + tx.round()) as usize;

            // TODO: fix this
            if texture_index >= TEXTURE.len() {
                println!("Wall texture index is out of bounds! {}", texture_index);
                println!("tx: {}, ty: {}", ty.round(), tx.round());
                texture_index = TEXTURE.len() - 1;
            }

            let color = (TEXTURE[texture_index] as f64 * 255.0 * shade) as u8;
            let color = color::Color::new(color, color, color, 255);

            for _i in 0..line_width {
                draw_pixel(
                    x + _i as f64, 
                    y as f64 + line_off, 
                    color
                );
            }

            ty += ty_step;
        }

        // // Draw walls as simple 8px lines
        // let color = (255.0 * shade) as u8;
        // let color = color::Color::new(color , color, color, color);
        // for i in 0..line_width {
        //     draw_line(x + i as f64, line_off, x + i as f64, line_off + line_height, color);
        // }

        // Draw the floors and the ceiling
        
        let mut y = line_off + line_height;
        while y < 320.0 {
            let dy = y - (320. / 2.0);
            let deg = deg_to_rad(ra);
            let ra_fix = cos(deg_to_rad(fix_ang(pa - ra)));

            // Draw the floor
            tx = px / 2.0 + cos(deg) * 158.0 * 32.0 / dy / ra_fix;
            ty = py / 2.0 - sin(deg) * 158.0 * 32.0 / dy / ra_fix;

            let map_index = (ty / 32.0 * mapx as f64 + tx / 32.0) as usize;
            
            let mut mp = 0;

            if map_index > Map::global().floor_data.len() {
                println!("Floor texture index is out of bounds! {}", map_index); // TODO: fix this
            } else {
                mp = Map::global().floor_data[map_index] * 32 * 32;
            }

            let texture_x = (tx as i32) & 31;
            let texture_y = (ty as i32) & 31;

            let texture_index = (texture_y * 32 + texture_x + mp) as usize;
            let color = (TEXTURE[texture_index] as f64 * 255.0) * 0.7;
            let color = color::Color::new(color as u8, color as u8, color as u8, 255);
            //let color = color::Color::new(255, 0, 0, 255);
            for _i in 0..line_width {
                draw_pixel(
                    x + _i as f64, 
                    y,
                    color
                );
            }

            // Draw the ceiling
            if map_index > Map::global().ceiling_data.len() {
                println!("Ceiling texture index is out of bounds! {}", map_index); // TODO: fix this
            } else {
                mp = Map::global().ceiling_data[map_index] * 32 * 32;
            }

            let mut color = color::Color::new(135, 206, 235, 255);
            if mp == 0 {
                let texture_index = (texture_y * 32 + texture_x + mp) as usize;
                let texture_color = (TEXTURE[texture_index] as f64 * 255.0) * 0.7;
                color = color::Color::new(
                    texture_color as u8, 
                    texture_color as u8, 
                    texture_color as u8, 
                    255
                );
            }
            
            for _i in 0..line_width {
                draw_pixel(
                    x + _i as f64, 
                    320.0 - y,
                    color
                );
            }

            y += 1.0;
        }
        

        ra = fix_ang(ra - 1.0);
    }
}

