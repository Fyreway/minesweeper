#![allow(dead_code)]
use std::time::Duration;

use sdl2::{
    event::Event,
    image::{self, InitFlag, LoadTexture, Sdl2ImageContext},
    render::{TextureCreator, WindowCanvas},
    ttf::{self, Sdl2TtfContext},
    video::WindowContext,
    EventPump, Sdl, VideoSubsystem,
};

mod button;

mod game;
use game::{GameState, Map, MapSize};

mod main_menu;

fn generate(size: MapSize) -> Map {
    let mut map = Map::new(size);
    map.generate_mines(&mut rand::thread_rng());
    map.generate_tiles();
    map
}

pub struct Context {
    sdl: Sdl,
    video_subsys: VideoSubsystem,
    image: Sdl2ImageContext,
    ttf: Sdl2TtfContext,
    canvas: WindowCanvas,
    tex_creator: TextureCreator<WindowContext>,
    event_pump: EventPump,
}
impl Context {
    fn new() -> Result<Self, String> {
        let sdl = sdl2::init()?;
        let video_subsys = sdl.video()?;
        let image = image::init(InitFlag::PNG)?;
        let ttf = ttf::init().map_err(|e| e.to_string())?;
        let win = video_subsys
            .window("Minesweeper", 800, 600)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;
        let canvas = win
            .into_canvas()
            .accelerated()
            .present_vsync()
            .build()
            .map_err(|e| e.to_string())?;
        let tex_creator = canvas.texture_creator();
        let event_pump = sdl.event_pump()?;
        Ok(Self {
            sdl,
            video_subsys,
            image,
            ttf,
            canvas,
            tex_creator,
            event_pump,
        })
    }
}

fn main() -> Result<(), String> {
    let mut ctx = Context::new()?;
    let tex = ctx.tex_creator.load_texture("res/spritesheet.png")?;
    // main_menu(&mut ctx)?;
    let mut map = generate(MapSize::Normal);
    'gameloop: loop {
        for e in ctx.event_pump.poll_iter() {
            match e {
                Event::Quit { .. } => break 'gameloop,
                _ => (),
            }
        }
        let mouse_state = ctx.event_pump.mouse_state();
        if mouse_state.left() {
            if let Some(tile) = map.inside(&mouse_state) {
                map.mine(tile, &mut Vec::new());
            }
        } else if mouse_state.right() {
            if let Some(tile) = map.inside(&mouse_state) {
                map.flag(tile);
            }
        }

        match map.check_state() {
            GameState::Win => todo!(),
            GameState::Lose => todo!(),
            GameState::Playing => (),
        }

        ctx.canvas.clear();
        map.render(&mut ctx.canvas, &tex)?;
        ctx.canvas.present();

        std::thread::sleep(Duration::from_nanos(1_000_000_000u64 / 60));
    }

    Ok(())
}
