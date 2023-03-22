struct Ray {
    x: f64,
    y: f64,
    offset_x: f64,
    offset_y: f64,
    dist_vertical: f64,
    dist_horizontal: f64,
    vx: f64,
    vy: f64,
}

impl Ray {
    pub fn new() -> Ray {
        Ray {
            x: 0.0,
            y: 0.0,
            offset_x: 0.0,
            offset_y: 0.0,
            dist_vertical = 100000.0,
            dist_horizontal = 100000.0,
            vx: 0.0,
            vy: 0.0,
        }
    }

    pub fn check_vertical_lines(&mut self, ray_index: i32, ray_angle: f64) {
        let (player_x, player_y) = Player::global().get_position();
        let map_grid_size = Map::global().size;
        let map_width = Map::global().x;

        let ray_angle_rad = deg_to_rad(ray_angle);
        let ray_angle_tan = ray_angle.tan();

        if ray_angle.cos() > 0.001 { // looking left
            self.x = ((player_x / map_grid_size) as i32 * map_grid_size) + map_grid_size;
            self.y = (player_x - self.x) * ray_angle_tan + player_y;
            self.offset_x = map_grid_size;
            self.offset_y = -self.offset_x * ray_angle_tan;
        } else if ray_angle.cos() < -0.001 { // looking right
            self.x = ((player_x / map_grid_size) as i32 * map_grid_size) - 0.0001;
            self.y = (player_x - self.x) * ray_angle_tan + player_y;
            self.offset_x = -map_grid_size;
            self.offset_y = -self.offset_x * ray_angle_tan;
        } else { // looking up or down
            self.x = player_x;
            self.y = player_y;
            dof = 8;
        }

        while dof < 8 {
            let on_map_x = self.x as i32 / map_grid_size;
            let on_map_y = self.y as i32 / map_grid_size;
            let map_position = on_map_y * map_width + on_map_x;

            if map_position > 0 && map_position < map_width * map_width && Map::global().data[map_position as usize] > 0 {
                dof = 8; // hit wall
                self.dist_vertical = ray_angle_rad.cos() * (self.x - player_x) - ray_angle_rad.sin() * (self.y - player_y);
            } else { // next line
                self.x += self.offset_x;
                self.y += self.offset_y;
                dof += 1;
            }
        }

        self.vx = self.x;
        self.vy = self.y;
    }

    pub fn check_horizontal_lines(&mut self, ray_index: i32, ray_angle: f64) {
        let (player_x, player_y) = Player::global().get_position();
        let map_grid_size = Map::global().size;
        let map_width = Map::global().x;

        let mut dof = 0;
        let ray_angle_rad = deg_to_rad(ray_angle);
        let ray_angle_a_tan = 1.0 / ray_angle.tan();

        if ray_angle.sin() > 0.001 { // looking down
            self.y = ((player_y / map_grid_size) as i32 * map_grid_size) + map_grid_size;
            self.x = (player_y - self.y) * ray_angle_a_tan + player_x;
            self.offset_y = map_grid_size;
            self.offset_x = -self.offset_y * ray_angle_a_tan;
        } else if ray_angle.sin() < -0.001 { // looking up
            self.y = ((player_y / map_grid_size) as i32 * map_grid_size) - 0.0001;
            self.x = (player_y - self.y) * ray_angle_a_tan + player_x;
            self.offset_y = -map_grid_size;
            self.offset_x = -self.offset_y * ray_angle_a_tan;
        } else { // looking left or right
            self.x = player_x;
            self.y = player_y;
            dof = 8;
        }
    }
}