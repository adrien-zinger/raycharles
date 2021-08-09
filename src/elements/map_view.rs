extern crate sdl2;
use sdl2::rect::Rect;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::render::Canvas;

fn map_value(s: u32, a1: u32, a2: u32, b1: u32, b2: u32) -> u32 {
  b1 + ((s - a1) * (b2 - b1) / (a2 - a1)) 
}

fn get_rect(x: usize, y: usize) -> Rect {
  Rect::new(
      map_value(x as u32, 0, 8, 0, 800) as i32,
      map_value(y as u32, 0, 6, 0, 600) as i32,
      map_value((x as u32) + 1, 0, 8, 0, 800) as u32,
      map_value((y as u32) + 1, 0, 6, 0, 600) as u32
  )
}

pub fn draw_map(canvas: &mut Canvas<Window>) {
  let map = [
    [1,1,1,1,1,1,1,1],
    [1,0,0,0,0,0,0,1],
    [1,1,0,0,1,0,1,1],
    [1,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,1],
    [1,1,1,1,1,1,1,1],
  ];
  for (y, &line) in map.iter().enumerate() {
      for (x, &w) in line.iter().enumerate() {
          if w == 1 {
              canvas.set_draw_color(Color::RGB(20, 20, 20));
          } else {
              canvas.set_draw_color(Color::RGB(200, 200, 200));
          }
          canvas.fill_rect(get_rect(x, y)).expect("Error writting rect");
      }
  }
}
