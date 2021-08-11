use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::cmp::min;
use std::cmp::max;

use crate::statics::colors::LIGHT;

fn get_m(a: &Point, b: &Point) -> f32 {
  let d = a.x - b.x;
  if d == 0 { 0.0 }
  else { (a.y - b.y) as f32 / d as f32 }
}

fn draw_function(a: &Point, b: &Point, canvas: &mut Canvas<Window>) {
  let m = get_m(a, b);
  let s = a.y as f32 - m * a.x as f32;
  let mut i = a.x as f32;
  let offset = 1.0;
  let mut p1 = Point::new(0,0);
  let mut p2 = Point::new(0,0);
  while i <= (b.x as f32 - offset) {
    let x1 = i;
    let y1 = m * x1 + s;
    let x2 = i + offset;
    let y2 = m * x2 + s;
    p1.x = x1 as i32;
    p1.y = y1 as i32;
    p2.x = x2 as i32;
    p2.y = y2 as i32;
    canvas.set_draw_color(LIGHT);
    canvas.draw_line(p1, p2).expect("Error writting point");
    i += 1.0;
  }
}

fn draw_vertical_line(a: &Point, b: &Point, canvas: &mut Canvas<Window>) {
  let mut i = min(a.y, b.y) as f32;
  while i <= max(a.y, b.y) as f32 {
    canvas.set_draw_color(LIGHT);
    canvas.draw_point(Point::new(a.x, i as i32)).expect("Error writting point");
    i += 0.05;
  }
}

pub fn draw(a: &Point, b: &Point, canvas: &mut Canvas<Window>) {
  if a.x == b.x {
    draw_vertical_line(a, b, canvas);
  } else if a.x < b.x {
    draw_function(a, b, canvas);
  } else {
    draw_function(b, a, canvas);
  }
  //1.0 / (1.0 + 1.00*d + 1.00*d2);
}