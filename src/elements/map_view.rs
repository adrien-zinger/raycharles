extern crate sdl2;
use sdl2::rect::Rect;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use std::vec::Vec;

fn map_value(s: u32, a1: u32, a2: u32, b1: u32, b2: u32) -> u32 {
  b1 + ((s - a1) * (b2 - b1) / (a2 - a1)) 
}

fn get_rect(x: usize, y: usize, mapw: usize, maph: usize) -> Rect {
  Rect::new(
      map_value(x as u32, 0, mapw as u32, 0, 800) as i32,
      map_value(y as u32, 0, maph as u32, 0, 600) as i32,
      map_value((x as u32) + 1, 0, mapw as u32, 0, 800) as u32,
      map_value((y as u32) + 1, 0, maph as u32, 0, 600) as u32
  )
}

pub fn draw_map(map: &Vec::<Vec::<i8>>, canvas: &mut Canvas<Window>) {
  canvas.set_draw_color(Color::RGB(200, 200, 200));
  canvas.clear();
  for y in 0..map.len() {
      for x in 0..map[y].len() {
          if map[y][x] == 1 {
              canvas.set_draw_color(Color::RGB(20, 20, 20));
              canvas.fill_rect(get_rect(x, y, map[0].len(), map.len())).expect("Error writting rect");
          }
      }
  }
}
