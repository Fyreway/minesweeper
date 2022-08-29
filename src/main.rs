#![allow(dead_code)]
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
use game::{Map, MapSize};

mod main_menu;
use main_menu::main_menu;

fn generate(tex_creator: &TextureCreator<WindowContext>, size: MapSize) -> Map {
    let mut map = Map::new(tex_creator, size);
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
    fn new() -> Self {
        let sdl = sdl2::init().expect("Could not start SDL");
        let video_subsys = sdl.video().expect("Could not start video subsystem");
        let image = image::init(InitFlag::PNG).expect("Could not start SDL_image");
        let ttf = ttf::init().expect("Could not start SDL_ttf");
        let win = video_subsys
            .window("Minesweeper", 800, 600)
            .position_centered()
            .build()
            .expect("Could not create window");
        let canvas = win
            .into_canvas()
            .accelerated()
            .present_vsync()
            .build()
            .expect("Could not create canvas");
        let tex_creator = canvas.texture_creator();
        let event_pump = sdl.event_pump().expect("Could not get event pump");
        Self {
            sdl,
            video_subsys,
            image,
            ttf,
            canvas,
            tex_creator,
            event_pump,
        }
    }
}

fn main() {
    let mut ctx = Context::new();
    let tex = ctx
        .tex_creator
        .load_texture("res/spritesheet.png")
        .expect("Could not load spritesheet");
    // main_menu(&mut ctx);
    let mut map = generate(&ctx.tex_creator, MapSize::Normal);
    'gameloop: loop {
        for e in ctx.event_pump.poll_iter() {
            match e {
                Event::Quit { .. } => break 'gameloop,
                _ => (),
            }
        }
        ctx.canvas.clear();
        map.render(&mut ctx.canvas, &tex);
        ctx.canvas.present();
    }
}
