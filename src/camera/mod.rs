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

//pub fn map(s: f64, a1: f64, a2: f64, b1: f64, b2: f64) -> f64 {
//  b1 + ((s - a1) * (b2 - b1) / (a2 - a1))
//}

fn dist_2d(a: Point, b: Point) -> f64 {
  let x1 = a.x as f64;
  let y1 = a.y as f64;
  let x2 = b.x as f64;
  let y2 = b.y as f64;
  let sqr = (x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1);
  sqr.abs().sqrt()
}

fn get_dir_2d(pos: &Point, angle_deg: f32) -> (f32, f32) {
  let mut a = angle_deg;
  let x = pos.x as f32;
  let y = pos.y as f32;
  if a > 359.0 { a -= 360.0; }
  else if a < 0.0 { a += 360.0; }
  let dirx = x + a.to_radians().cos() * 10.0;
  let diry = y - a.to_radians().sin() * 10.0;
  (dirx, diry)
}

pub fn draw_vision_2d(walls: &Vec::<Wall2d>, player: &Player, canvas: &mut Canvas<Window>) {
  let pos = Point::new(player.x as i32, player.y as i32);
  let mut angle_offset = -45.0;
  while angle_offset < 45.0 {
    let dir = get_dir_2d(&pos, player.pa as f32 + angle_offset);
    let mut dest = Point::new(0, 0);
    let mut dist = f64::MAX;
    for wall in walls {
      let p = cast_wall_2d(wall, &pos, &dir.0, &dir.1);
      if p.is_some() {
        let wall_cast = p.unwrap();
        let d = dist_2d(pos, wall_cast);
        if d < dist {
          dist = d;
          dest.x = wall_cast.x;
          dest.y = wall_cast.y;
        }
      }
    }
    if dest.x != 0 && dest.y != 0 {
      canvas.set_draw_color(Color::RGB(105, 25, 25));
      canvas.draw_line(pos, dest).expect("Error writting line vision 2d");
    }
    angle_offset += 0.1;
  }
}
