mod math;
mod color;
mod screen;
mod draw;
mod player;
mod map;
mod input;
mod ray;
mod texture;

use std::panic;
use std::task::Poll;
use once_cell::sync::OnceCell;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
extern crate console_error_panic_hook;

#[cfg(target_arch = "wasm32")]
use web_sys::console;

use crate::draw::{fill};
use crate::screen::Screen;
use crate::player::Player;
use crate::map::Map;
use crate::input::Input;
use crate::ray::cast_ray;

static SCREEN_INSTANCE: OnceCell<Screen> = OnceCell::new();
static mut PLAYER_INSTANCE: OnceCell<Player> = OnceCell::new();
static mut MAP_INSTANCE: OnceCell<Map> = OnceCell::new();
static mut INPUT_INSTANCE: OnceCell<Input> = OnceCell::new();

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn init_lib() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            console::log_1(&"Running on wasm platform".into());
            panic::set_hook(Box::new(console_error_panic_hook::hook));
        } else {
            println!("Running on native platform");
            panic::set_hook(Box::new(|info| {
                eprintln!("{}", info);
            }));
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn setup(width: i32, height: i32) {
    let screen = Screen::new(width, height);
    SCREEN_INSTANCE.set(screen).unwrap();

    let player = Player::new(100.0, 100.0);
    unsafe { PLAYER_INSTANCE.set(player).unwrap() };

    let map = Map::new();
    unsafe { MAP_INSTANCE.set(map).unwrap() };

    let input = Input::new();
    unsafe { INPUT_INSTANCE.set(input).unwrap() };
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn get_output_buffer_pointer() -> *const u8 {
    let pointer: *const u8;
    pointer = Screen::global().output_buffer.lock().unwrap().as_ptr();
    return pointer;
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn key_down(key: u8) {
    Input::global().key_down(key);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn key_up(key: u8) {
    Input::global().key_up(key);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn present(delta_time: f64) {
    let color = color::Color::new(76, 76, 76, 255);
    fill(color);

    if Input::global().up {
        Player::global().translate(1, 0, delta_time);
    }

    if Input::global().down {
        Player::global().translate(-1, 0, delta_time);
    }

    if Input::global().strafe_left {
        Player::global().translate(0, 1, delta_time);
    }

    if Input::global().strafe_right {
        Player::global().translate(0, -1, delta_time);
    }

    if Input::global().left {
        Player::global().rotate(-Player::global().turn_speed, delta_time);
    }

    if Input::global().right {
        Player::global().rotate(Player::global().turn_speed, delta_time);
    }

    if Input::global().open_door {
        let player = Player::global();
        let map: &mut Map = Map::global();

        let mut xo; if player.dx < 0.0 { xo=-25;} else{ xo=25; }
        let mut yo; if player.dy < 0.0 { yo=-25;} else{ yo=25; } 
        let ipx = player.x / 64.0;
        let ipx_add_xo = (player.x + xo as f64 ) / 64.0;
        let ipy = player.y / 64.0;
        let ipy_add_yo=(player.y + yo as f64) / 64.0;

        let map_index = (ipy_add_yo * map.x as f64 + ipx_add_xo) as usize;
        if map.data[map_index] == 4 { map.set_wall(map_index, 0); }


    }

    Map::global().draw_2d();

    cast_ray();

    Player::global().draw_2d();

}

