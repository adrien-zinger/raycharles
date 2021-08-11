extern crate sdl2;
use sdl2::rect::Rect;
use sdl2::video::Window;
use sdl2::render::Canvas;
use std::vec::Vec;

use crate::elements::tools::map_value;
use crate::statics::limits::LIMITS_2D;
use crate::statics::colors;

fn get_rect(x: usize, y: usize, mapw: usize, maph: usize) -> Rect {
  Rect::new(
    map_value(x as u32, 0, mapw as u32, LIMITS_2D.0, LIMITS_2D.2) as i32,
    map_value(y as u32, 0, maph as u32, LIMITS_2D.1, LIMITS_2D.3) as i32,
    map_value((x as u32) + 1, 0, mapw as u32, LIMITS_2D.0, LIMITS_2D.2) as u32,
    map_value((y as u32) + 1, 0, maph as u32, LIMITS_2D.1, LIMITS_2D.3) as u32
  )
}

pub fn draw_map(map: &Vec::<Vec::<i8>>, canvas: &mut Canvas<Window>) {
  for y in 0..map.len() {
    for x in 0..map[y].len() {
      if map[y][x] == 1 {
        canvas.set_draw_color(colors::WALL);
      } else {
        canvas.set_draw_color(colors::FLOOR);
      }
      canvas.fill_rect(get_rect(x, y, map[0].len(), map.len())).expect("Error writting rect");
    }
  }
}
