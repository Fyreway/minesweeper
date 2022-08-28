#![allow(dead_code)]
use sdl2::{
    event::Event,
    image::{self, InitFlag},
    ttf,
};
use std::{path::Path, time::Duration};

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
    let _img = image::init(InitFlag::PNG).expect("Could not start SDL_image");
    let ttf = ttf::init().expect("Could not start SDL_ttf");
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
    let tex_creator = canvas.texture_creator();
    let mut events = sdl.event_pump().expect("Could not get event pump");
    let small_btn = Button::new(
        -1,
        300,
        64,
        5,
        &tex_creator,
        &Path::new("res/button.png"),
        "Small".to_string(),
        &ttf.load_font("res/font/opensans.ttf", 40)
            .expect("Could not load font"),
    );
    let normal_btn = Button::new(
        -1,
        400,
        64,
        5,
        &tex_creator,
        &Path::new("res/button.png"),
        "Normal".to_string(),
        &ttf.load_font("res/font/opensans.ttf", 40)
            .expect("Could not load font"),
    );
    let large_btn = Button::new(
        -1,
        500,
        64,
        5,
        &tex_creator,
        &Path::new("res/button.png"),
        "Large".to_string(),
        &ttf.load_font("res/font/opensans.ttf", 40)
            .expect("Could not load font"),
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

        small_btn
            .render(&mut canvas)
            .expect("Could not render button");
        normal_btn
            .render(&mut canvas)
            .expect("Could not render button");
        large_btn
            .render(&mut canvas)
            .expect("Could not render button");

        canvas.present();
        std::thread::sleep(Duration::from_nanos(1_000_000_000u64 / 60));
    }
}
