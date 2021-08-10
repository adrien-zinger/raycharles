extern crate sdl2;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use std::vec::Vec;
use crate::elements::player::Player;

use crate::elements::wall::Wall2d;
mod casting;
use casting::cast_wall_2d;

pub fn draw_vision_2d(walls: &Vec::<Wall2d>, player: &Player, canvas: &mut Canvas<Window>) {
  for wall in walls {
    let p = cast_wall_2d(wall, player);
    if p.is_some() {
      println!("cast");
      canvas.set_draw_color(Color::RGB(105, 25, 25));
      canvas.draw_line(Point::new(player.x as i32, player.y as i32), p.unwrap());
    }
  }
}
