extern crate sdl2;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use std::f64::consts::PI;

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

pub fn lim_pos(p: f32, max: i32) -> u32 {
  if p > max as f32 {
    return max as u32;
  } else if p < 0.0 {
    return 0;
  } else {
    return p as u32;
  }
}

fn rad(a: i32) -> f64 {
  (a as f64) * PI / 180.0
}

impl Player {
  pub fn left(&mut self) {
    self.pa +=5;
    self.set_pd();
  }

  pub fn right(&mut self) {
    self.pa -= 5;
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
      return lim_pos((self.x as f32) + self.pdx * 5.0, 800);
    } else {
      return lim_pos((self.x as f32) - self.pdx * 5.0, 800);
    }
  }

  pub fn diry(&mut self, forward: bool) -> u32 {
    if forward {
      return lim_pos((self.y as f32) + self.pdy * 5.0, 600);
    } else {
      return lim_pos((self.y as f32) - self.pdy * 5.0, 600);
    }
  }

  fn set_pd(&mut self) {
    lim_pa(&mut self.pa);
    self.pdx = rad(self.pa).cos() as f32;
    self.pdy = -rad(self.pa).sin() as f32;
    println!("pd x y {} {} {}", rad(self.pa).cos(), self.pdx, self.pdy);
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
        canvas.set_draw_color(Color::RGB(105, 25, 25));
        canvas.draw_point(Point::new(i, j)).expect("Error writting point");
      }
    }
  }
  canvas.set_draw_color(Color::RGB(105, 25, 25));
  canvas.draw_line(Point::new(x, y), Point::new(x + ((player.pdx * 20.0) as i32), y + ((player.pdy * 20.0) as i32)))
    .expect("Error writting line view");
}
