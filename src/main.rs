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
use elements::wall;
mod camera;
mod parser;

struct Game {
    canvas: Canvas<Window>,
    walls_2d: Vec::<wall::Wall2d>,
    map: Vec::<Vec<i8>>,
    player: elements::player::Player
}

fn draw(game: &mut Game) {
    draw_map(&game.map, &mut game.canvas);
    draw_player(&game.player, &mut game.canvas);
    camera::draw_vision_2d(&game.walls_2d, &game.player, &mut game.canvas);
    wall::draw_walls_2d(&game.walls_2d, &mut game.canvas);
    game.canvas.present();
}

fn run(canvas: Canvas<Window>, mut event_pump: EventPump) {
    let map = parser::read("./src/local");
    let mut game = Game {
        canvas,
        map: map.clone(),
        player: elements::player::Player {
            x: 100,
            y: 100,
            pdx: 0.5,
            pdy: 0.5,
            pa: 0
        },
        walls_2d: wall::get_walls_2d(&map)
    };

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    game.player.right();
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    game.player.left();
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    game.player.down();
                },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    game.player.up();
                },
                _ => {}
            }
        }

        draw(&mut game);
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
