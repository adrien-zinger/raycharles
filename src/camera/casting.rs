extern crate sdl2;
use sdl2::rect::Point;

use crate::elements::wall::Wall2d;
use crate::elements::player::Player;

pub fn cast_wall_2d(wall: &Wall2d, player: &Player) -> Option<Point> {
  let x1 = wall.a.x as f32;
  let y1 = wall.a.y as f32;
  let x2 = wall.b.x as f32;
  let y2 = wall.b.y as f32;
  let x3 = player.x as f32;
  let y3 = player.y as f32;
  let x4 = x3 + player.pdx * 20.0;
  let y4 = y3 + player.pdy * 20.0;
  let d = ((x1 - x2) * (y3 - y4)) - ((y1 - y2) * (x3 - x4));
  if d == 0.0 { return None; }

  let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / d;
  let u = 0.0 - ((x2 - x1) * (y1 - y3) - (y2 - y1) * (x1 - x3)) / d;
  
  if 0.0 < t && t < 1.0 && u < 0.0 {
    return Some(Point::new((x1 + (t * (x2 - x1))) as i32, (y1 + (t * (y2 - y1))) as i32));
  }

  None
}
