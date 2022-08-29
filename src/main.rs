#![allow(dead_code)]
use sdl2::{
    event::Event,
    image::{self, InitFlag, Sdl2ImageContext},
    mouse::MouseState,
    render::{TextureCreator, WindowCanvas},
    ttf::{self, Sdl2TtfContext},
    video::WindowContext,
    EventPump, Sdl, VideoSubsystem,
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

struct Context {
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

struct MainMenu<'a> {
    btns: Vec<Button<'a>>,
}
impl MainMenu<'_> {
    fn new<'a>(
        ttf: &Sdl2TtfContext,
        tex_creator: &'a TextureCreator<WindowContext>,
    ) -> MainMenu<'a> {
        MainMenu {
            btns: vec![
                Button::new(
                    -1,
                    300,
                    64,
                    5,
                    tex_creator,
                    &Path::new("res/button.png"),
                    "Small".to_string(),
                    &ttf.load_font("res/font/opensans.ttf", 40)
                        .expect("Could not load font"),
                ),
                Button::new(
                    -1,
                    400,
                    64,
                    5,
                    tex_creator,
                    &Path::new("res/button.png"),
                    "Normal".to_string(),
                    &ttf.load_font("res/font/opensans.ttf", 40)
                        .expect("Could not load font"),
                ),
                Button::new(
                    -1,
                    500,
                    64,
                    5,
                    tex_creator,
                    &Path::new("res/button.png"),
                    "Large".to_string(),
                    &ttf.load_font("res/font/opensans.ttf", 40)
                        .expect("Could not load font"),
                ),
            ],
        }
    }

    fn handle_clicks(&self, m: &MouseState) {
        if self.btns[0].is_clicked(m) {
            println!("Small button clicked");
        } else if self.btns[1].is_clicked(m) {
            println!("Normal button clicked");
        } else if self.btns[2].is_clicked(m) {
            println!("Large button clicked");
        }
    }

    fn render(&self, canvas: &mut WindowCanvas) {
        canvas.clear();

        for btn in self.btns.as_slice() {
            btn.render(canvas).expect("Could not render button");
        }

        canvas.present();
    }
}

fn main_menu(ctx: &mut Context) {
    let main_menu = MainMenu::new(&ctx.ttf, &ctx.tex_creator);

    'top: loop {
        for e in ctx.event_pump.poll_iter() {
            match e {
                Event::Quit { .. } => break 'top,
                _ => (),
            }
        }
        let mouse_state = ctx.event_pump.mouse_state();
        if mouse_state.left() {
            main_menu.handle_clicks(&mouse_state);
        }

        main_menu.render(&mut ctx.canvas);
        std::thread::sleep(Duration::from_nanos(1_000_000_000u64 / 60));
    }
}

fn main() {
    let mut ctx = Context::new();
    main_menu(&mut ctx);
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
}
