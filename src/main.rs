#![allow(dead_code)]
use sdl2::{event::Event, image::InitFlag};
use std::{path::PathBuf, time::Duration};

mod button;
use button::Button;

mod game;
use game::{Map, MapSize};

fn generate(size: MapSize) -> Map {
    let mut map = Map::new(size);
    map.generate_mines(&mut rand::thread_rng());
    map.generate_tiles();
    map
}

fn main() {
    let sdl = sdl2::init().expect("Could not start SDL");
    let video = sdl.video().expect("Could not start video subsystem");
    let _img = sdl2::image::init(InitFlag::PNG).expect("Could not start SDL_image");
    let window = video
        .window("Minesweeper", 800, 600)
        .position_centered()
        .build()
        .expect("Could not create window");
    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .expect("Could not create canvas");
    let mut tex_creator = canvas.texture_creator();
    let mut events = sdl.event_pump().expect("Could not get event pump");
    let btn = Button::new(
        -1,
        -1,
        128,
        5,
        &mut tex_creator,
        PathBuf::from("res/button.png"),
    );

    'gameloop: loop {
        for e in events.poll_iter() {
            match e {
                Event::Quit { .. } => break 'gameloop,
                _ => (),
            }
        }
        // match map.check_state() {
        //     GameState::Win => {
        //         // TODO: Display win state
        //         break 'gameloop;
        //     }
        //     GameState::Lose => {
        //         // TODO: Display lose state
        //         break 'gameloop;
        //     }
        //     GameState::Playing => {}
        // }

        canvas.clear();

        btn.render(&mut canvas).expect("Could not render button");

        canvas.present();
        std::thread::sleep(Duration::from_nanos(1_000_000_000u64 / 60));
    }
}
