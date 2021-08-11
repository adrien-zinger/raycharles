pub fn draw_circle(x: i32 y: i32, canvas: &mut Canvas<Window>) {
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
}
