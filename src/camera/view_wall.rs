use crate::elements::tools::map_value;
use crate::statics::limits::VISION;
use crate::statics::limits::LIMITS;
//use crate::statics::limits::LIMITS_2D;
use crate::statics::colors::WALL;
use crate::statics::colors::FLOOR;

use std::cmp::min;
use std::cmp::max;

use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

fn attenuation(dist: f64) -> Color {
  let mult = 500.0 / (1.0 + 2.0 * dist + dist * dist);
  Color::RGB(
    (WALL.r as f64 * mult) as u8,
    (WALL.g as f64 * mult) as u8,
    (WALL.b as f64 * mult) as u8
  )
}

fn dist_2d(a: &Point, b: &Point) -> f64 {
  let x1 = min(a.x, b.x) as f64;
  let y1 = min(a.y, b.y) as f64;
  let x2 = max(a.x, b.x) as f64;
  let y2 = max(a.y, b.y) as f64;
  let sqr = (x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1);
  sqr.sqrt()
}

fn fill_rect_from_center(x: i32, w: u32, h: u32, color: Color, canvas: &mut Canvas<Window>) {
  //let mut y = 0;
  let middle = LIMITS.1 / 2;
  let h2 = h / 2;
  //if x < LIMITS_2D.2 as i32 {
  //  y = LIMITS_2D.3 as i32;
  //}
  //let y2 = h2 - y as u32;
  canvas.set_draw_color(FLOOR);
  canvas.fill_rect(Rect::new(x, 0, w, (middle - h2) as u32)).expect("Error writting rect");
  canvas.set_draw_color(color);
  canvas.fill_rect(Rect::new(x, (middle - h2) as i32, w, h2)).expect("Error writting rect");
  canvas.fill_rect(Rect::new(x, middle as i32, w, h2)).expect("Error writting rect");
  canvas.set_draw_color(FLOOR);
  canvas.fill_rect(Rect::new(x, (middle + h2) as i32, w, (LIMITS.1 - middle - h2) as u32)).expect("Error writting rect");
}

fn draw_wall_slice(x: i32, w: f32, dist: f64, canvas: &mut Canvas<Window>) {
  let n = w / 10.0;
  for i in 0..10 {
    let offset = i as f32 * n;
    fill_rect_from_center(x + offset as i32, (w + offset) as u32, LIMITS.1 / dist as u32 * 4, attenuation(dist), canvas);
  }
}

pub fn draw(a: &Point, b: &Point, angle: f32, canvas: &mut Canvas<Window>) {
  let pa = angle + VISION / 2.0;
  let x = map_value(pa as u32, 0, VISION as u32, 0, LIMITS.0);
  let w = (LIMITS.0 as f32 / VISION) + 1.;
  draw_wall_slice(x as i32, w, dist_2d(a, b), canvas);
}
