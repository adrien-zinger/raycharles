mod elements;

use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use std::vec::Vec;

use elements::wall::Wall_2d;
use elements::player::Player;

pub fn cast_wall_2d(wall: &Wall_2d, player: &Player) -> Option<Point> {
  let x1 = wall.a.x;
  let y1 = wall.a.y;
  let x2 = wall.b.x;
  let y2 = wall.b.y;

  let x3 = player.x;
  let y3 = player.y;
  let x4 = player.dirx(true);
  let y4 = player.diry(true);

  let d = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
  if d == 0 { return None; }

  let t = ((x1 - x3) * (y1 - y4) - (y1 - y3) * (x3 - x4)) / d;
  let u = -((x2 - x1) * (y1 - y3) - (y2 - y1) * (x1 - x3)) / d;
  if 0 < t && t < 1 && u < 0 {
    return Some(Point::new(x1 + t * (x2 + x1), y1 + t * (y2 - y1)));
  }

  None
}
