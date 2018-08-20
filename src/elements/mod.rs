extern crate sdl2;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub mod tools;
pub mod player;
mod map_view;
pub mod wall;

pub fn draw_player(player: &player::Player, canvas: &mut Canvas<Window>) {
  player::draw_player(player, canvas);
}

pub fn draw_map(map: &Vec::<Vec::<i8>>, canvas: &mut Canvas<Window>) {
  map_view::draw_map(map, canvas);
}
