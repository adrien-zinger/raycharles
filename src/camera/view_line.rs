use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::cmp::min;
use std::cmp::max;
use sdl2::pixels::Color;

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
  let mult = 10000.0 / (1.0 + 2.0 * dist + dist * dist);
  Color::RGB(
    (LIGHT.r as f64 * mult) as u8,
    (LIGHT.g as f64 * mult) as u8,
    (LIGHT.b as f64 * mult) as u8
  )
}

fn draw_function(a: &Point, b: &Point, start: &Point, canvas: &mut Canvas<Window>) {
  let m = get_m(a, b);
  let s = a.y as f32 - m * a.x as f32;
  let mut i = a.x as f32;
  let offset = 4.0;
  let mut p1 = Point::new(0,0);
  let mut p2 = Point::new(0,0);
  let max = b.x as f32;
  while i < max {
    let x1 = i;
    let y1 = m * x1 + s;
    let x2 = i + offset;
    let y2 = m * x2 + s;
    p1.x = x1 as i32;
    p1.y = y1 as i32;
    p2.x = x2 as i32;
    p2.y = y2 as i32;
    canvas.set_draw_color(attenuation(dist_2d(&p1, start)));
    canvas.draw_line(p1, p2).expect("Error writting point");
    if i + offset > max {
      let x1 = x2;
      let y1 = y2;
      let x2 = max;
      let y2 = m * x2 + s;
      p1.x = x1 as i32;
      p1.y = y1 as i32;
      p2.x = x2 as i32;
      p2.y = y2 as i32;
      canvas.set_draw_color(attenuation(dist_2d(&p1, start)));
      canvas.draw_line(p1, p2).expect("Error writting point");
    }
    i += offset;
  }
}

fn draw_vertical_line(a: &Point, b: &Point, start: &Point, canvas: &mut Canvas<Window>) {
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
  //1.0 / (1.0 + 1.00*d + 1.00*d2);
}