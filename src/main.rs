extern crate sdl2;

use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use std::time::Duration;
use sdl2::video::Window;

mod elements;
use elements::draw_map;
use elements::draw_player;
mod parser;

fn run(mut canvas: Canvas<Window>, mut event_pump: EventPump) {
    let mut player = elements::player::Player {
        x: 100,
        y: 100,
        pdx: 1.0,
        pdy: 1.0,
        pa: 0
    };
    let map: Vec::<Vec<i8>> = parser::read("./src/local");
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    player.right();
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    player.left();
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    player.down();
                },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    player.up();
                },
                _ => {}
            }
        }
        draw_map(&map.clone(), &mut canvas);
        draw_player(&player, &mut canvas);
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();
    run(window.into_canvas().build().unwrap(),
        sdl_context.event_pump().unwrap())
}
