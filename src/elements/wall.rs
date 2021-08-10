extern crate sdl2;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;

use crate::elements::tools::map_value;

#[derive(Debug, Clone)]
pub struct Wall2d {
  a: Point,
  b: Point
}

fn get_wall(x1: usize, y1: usize, x2: usize, y2: usize, map_width: usize, map_height: usize) -> Wall2d {
  let ax1 = map_value(x1 as u32, 0 as u32, map_width as u32, 0, 800);
  let ay1 = map_value(y1 as u32, 0 as u32, map_height as u32, 0, 600);
  let ax2 = map_value(x2 as u32, 0 as u32, map_width as u32, 0, 800);
  let ay2 = map_value(y2 as u32, 0 as u32, map_height as u32, 0, 600);
  Wall2d {
    a: Point::new(ax1 as i32, ay1 as i32),
    b: Point::new(ax2 as i32, ay2 as i32)
  }
}

pub fn get_walls_2d(map: &Vec::<Vec::<i8>>) -> Vec<Wall2d> {
  let mut ret: Vec<Wall2d> = Vec::new();
  for y in 0..map.len() {
    for x in 0..map[y].len() {
      if map[y][x] == 1 {
        ret.push(get_wall(x, y, x + 1, y, map[y].len(), map.len()));
        ret.push(get_wall(x + 1, y, x + 1, y + 1, map[y].len(), map.len()));
        ret.push(get_wall(x + 1, y + 1, x, y + 1, map[y].len(), map.len()));
        ret.push(get_wall(x, y + 1, x, y, map[y].len(), map.len()));
        if x == 0 && y == 0 {
          let w = get_wall(x, y + 1, x + 1, y + 1, map[y].len(), map.len());
          println!("{} {}", w.a.x, w.a.y);
          println!("{} {}", w.b.x, w.b.y);
        }
      }
    }
  }
  ret
}

pub fn draw_walls_2d(walls: &Vec<Wall2d>, canvas: &mut Canvas<Window>) {
  canvas.set_draw_color(Color::RGB(65, 85, 25));
  for wall in walls {
    canvas.draw_line(wall.a, wall.b)
      .expect("Error writting wall");
  }
}