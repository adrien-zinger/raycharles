use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::cmp::min;
use std::cmp::max;
use sdl2::pixels::Color;
use std::vec::Vec;

use crate::statics::colors::LIGHT;

fn get_m(a: &Point, b: &Point) -> f32 {
  let d = a.x - b.x;
  (a.y - b.y) as f32 / d as f32
}

fn dist_2d(a: &Point, b: &Point) -> f64 {
  let x1 = min(a.x, b.x) as f64;
  let y1 = min(a.y, b.y) as f64;
  let x2 = max(a.x, b.x) as f64;
  let y2 = max(a.y, b.y) as f64;
  let sqr = (x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1);
  sqr.sqrt()
}

fn attenuation(dist: f64) -> Color {
  let mult = 500.0 / (1.0 + 2.0 * dist + dist * dist);
  Color::RGB(
    (LIGHT.r as f64 * mult) as u8,
    (LIGHT.g as f64 * mult) as u8,
    (LIGHT.b as f64 * mult) as u8
  )
}

fn get_points(x1: f32, x2: f32) -> Vec::<f32> {
  let d = (x1 - x2).abs();
  let offset = d / 64.0;
  let mut ret: Vec::<f32> = Vec::new();
  for i in 0..65 {
    ret.push(x1 + i as f32 * offset);
  }
  ret
}

fn draw_function(
    a: &Point,
    b: &Point,
    start: &Point,
    canvas: &mut Canvas<Window>) {
  let mut p1 = Point::new(0,0);
  let mut p2 = Point::new(0,0);
  let m = get_m(a, b);
  let s = a.y as f32 - m * a.x as f32;
  let points = get_points(a.x as f32, b.x as f32);
  for i in 0..points.len() - 1 {
    p1.x = (points[i]) as i32;
    p1.y = (m * points[i] + s) as i32;
    p2.x = points[i + 1] as i32;
    p2.y = (m * points[i + 1] + s) as i32;
    canvas.set_draw_color(attenuation(dist_2d(&p1, start)));
    canvas.draw_line(p1, p2).expect("Error writting point");
  }
}

fn draw_vertical_line(
    a: &Point,
    b: &Point,
    start: &Point,
    canvas: &mut Canvas<Window>) {
  let mut i = a.y as f32;
  let offset = 4.0;
  let mut p1 = Point::new(0,0);
  let mut p2 = Point::new(0,0);
  while i <= b.y as f32 {
    p1.x = a.x;
    p2.x = b.x;
    p1.y = i as i32;
    p2.y = (i + offset) as i32;
    canvas.set_draw_color(attenuation(dist_2d(&p1, start)));
    canvas.draw_line(p1, p2).expect("Error writting point");
    i += offset;
  }
}

pub fn draw(a: &Point, b: &Point, canvas: &mut Canvas<Window>) {
  if a.x == b.x {
    if a.y < b.y {
      draw_vertical_line(a, b, a, canvas);
    } else {
      draw_vertical_line(b, a, a, canvas);
    }
  } else if a.x < b.x {
    draw_function(a, b, a, canvas);
  } else {
    draw_function(b, a, a, canvas);
  }
}
