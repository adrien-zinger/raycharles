extern crate sdl2;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub mod player;
mod map_view;

pub fn draw_player(player: &player::Player, canvas: &mut Canvas<Window>) {
  player::draw_player(player, canvas);
}

pub fn draw_map(canvas: &mut Canvas<Window>) {
  map_view::draw_map(canvas);
}
