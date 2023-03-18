mod math;
mod color;
mod screen;
mod draw;
mod player;
mod map;
mod input;
mod ray;

use std::panic;
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
static MAP_INSTANCE: OnceCell<Map> = OnceCell::new();
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
    MAP_INSTANCE.set(map).unwrap();

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
pub fn present(_dt: f64) {
    let color = color::Color::new(76, 76, 76, 255);
    fill(color);

    if Input::global().up {
        Player::global().move_foward();
    }

    if Input::global().down {
        Player::global().move_backward();
    }

    if Input::global().left {
        Player::global().rotate(-Player::global().turn_speed);
    }

    if Input::global().right {
        Player::global().rotate(Player::global().turn_speed);
    }

    Map::global().draw_2d();

    cast_ray();

    Player::global().draw_2d();

}

