extern crate sdl2;
use sdl2::rect::Point;

use crate::elements::wall::Wall2d;
use crate::elements::player::Player;
use crate::elements::player::lim_pos;

pub fn cast_wall_2d(wall: &Wall2d, player: &Player) -> Option<Point> {
  let x1 = wall.a.x as i32;
  let y1 = wall.a.y as i32;
  let x2 = wall.b.x as i32;
  let y2 = wall.b.y as i32;
  let x3 = player.x as i32;
  let y3 = player.y as i32;
  let x4 = lim_pos((player.x as f32) + player.pdx * 5.0, 800) as i32;
  let y4 = lim_pos((player.y as f32) + player.pdy * 5.0, 600) as i32;
  let d = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
  if d == 0 { return None; }

  let t = ((x1 - x3) * (y1 - y4) - (y1 - y3) * (x3 - x4)) / d;
  let u = 0 - ((x2 - x1) * (y1 - y3) - (y2 - y1) * (x1 - x3)) / d;
  println!("{} {}", t, u);
  if 0 < t && t < 1 && u < 0 {
    println!("ok");
    return Some(Point::new((x1 + t * (x2 + x1)) as i32, (y1 + t * (y2 - y1)) as i32));
  }

  None
}
