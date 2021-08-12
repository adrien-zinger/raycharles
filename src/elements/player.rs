extern crate sdl2;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::f64::consts::PI;

use crate::statics::limits::LIMITS_2D;
use crate::statics::colors;

pub struct Player {
  pub x: u32,
  pub y: u32,
  pub pdx: f32,
  pub pdy: f32,
  pub pa: i32
}

fn lim_pa(pa: &mut i32) {
  if *pa > 359 {
    *pa -= 360;
  } else if *pa < 0 {
    *pa += 360;
  }
}

pub fn lim_pos(p: f32, min: i32, max: i32) -> u32 {
  if p > max as f32 {
    return max as u32;
  } else if p < min as f32 {
    return min as u32;
  } else {
    return p as u32;
  }
}

fn rad(a: i32) -> f64 {
  (a as f64) * PI / 180.0
}

impl Player {
  pub fn left(&mut self) {
    self.pa += 1;
    self.set_pd();
  }

  pub fn right(&mut self) {
    self.pa -= 1;
    self.set_pd();
  }

  pub fn down(&mut self) {
    self.x = self.dirx(false);
    self.y = self.diry(false);
  }

  pub fn up(&mut self) {
    self.x = self.dirx(true);
    self.y = self.diry(true);
  }

  pub fn dirx(&mut self, forward: bool) -> u32 {
    if forward {
      return lim_pos((self.x as f32) + self.pdx * 5.0,
        LIMITS_2D.0 as i32,
        LIMITS_2D.2 as i32,
      );
    } else {
      return lim_pos((self.x as f32) - self.pdx * 5.0,
        LIMITS_2D.0 as i32,
        LIMITS_2D.2 as i32,
      );
    }
  }

  pub fn diry(&mut self, forward: bool) -> u32 {
    if forward {
      return lim_pos((self.y as f32) + self.pdy * 5.0,
        LIMITS_2D.1 as i32,
        LIMITS_2D.3 as i32,
      );
    } else {
      return lim_pos((self.y as f32) - self.pdy * 5.0,
        LIMITS_2D.1 as i32,
        LIMITS_2D.3 as i32,
      );
    }
  }

  fn set_pd(&mut self) {
    lim_pa(&mut self.pa);
    self.pdx = rad(self.pa).cos() as f32;
    self.pdy = -rad(self.pa).sin() as f32;
  }
}

pub fn draw_player(player: &Player, canvas: &mut Canvas<Window>) {
  let x = player.x as i32;
  let y = player.y as i32;
  let rad = 5;
  for i in (x - rad)..(x + rad) {
    for j in (y - rad)..(y + rad) {
      let dx = ((i-x) as i32).abs();
      let dy = ((j-y) as i32).abs();
      if dx*dx + dy*dy <= rad*rad {
        canvas.set_draw_color(colors::PLAYER);
        canvas.draw_point(Point::new(i, j)).expect("Error writting point");
      }
    }
  }
}
