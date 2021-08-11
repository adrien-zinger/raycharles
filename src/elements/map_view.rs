extern crate sdl2;
use sdl2::rect::Rect;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use std::vec::Vec;

use crate::elements::tools::map_value;
use crate::s_limits::LIMITS_2D;

fn get_rect(x: usize, y: usize, mapw: usize, maph: usize) -> Rect {
  Rect::new(
    map_value(x as u32, 0, mapw as u32, LIMITS_2D.0, LIMITS_2D.2) as i32,
    map_value(y as u32, 0, maph as u32, LIMITS_2D.1, LIMITS_2D.3) as i32,
    map_value((x as u32) + 1, 0, mapw as u32, LIMITS_2D.0, LIMITS_2D.2) as u32,
    map_value((y as u32) + 1, 0, maph as u32, LIMITS_2D.1, LIMITS_2D.3) as u32
  )
}

pub fn draw_map(map: &Vec::<Vec::<i8>>, canvas: &mut Canvas<Window>) {
  canvas.set_draw_color(Color::RGB(200, 200, 200));
  canvas.clear();
  for y in 0..map.len() {
    for x in 0..map[y].len() {
      if map[y][x] == 1 {
        canvas.set_draw_color(Color::RGB(20, 20, 20));
      } else {
        canvas.set_draw_color(Color::RGB(200, 200, 200));
      }
      canvas.fill_rect(get_rect(x, y, map[0].len(), map.len())).expect("Error writting rect");
    }
  }
}
