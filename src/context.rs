use resource::{resource, Resource};
use sdl2::{
    image::{self, InitFlag, Sdl2ImageContext},
    render::{TextureCreator, WindowCanvas},
    rwops::RWops,
    surface::Surface,
    ttf::{self, Sdl2TtfContext},
    video::WindowContext,
    EventPump, Sdl, VideoSubsystem,
};

pub struct Context {
    pub sdl: Sdl,
    pub video_subsys: VideoSubsystem,
    pub image: Sdl2ImageContext,
    pub ttf: Sdl2TtfContext,
    pub canvas: WindowCanvas,
    pub tex_creator: TextureCreator<WindowContext>,
    pub event_pump: EventPump,
    pub font_res: Resource<[u8]>,
}

impl Context {
    pub fn new() -> Result<Self, String> {
        let sdl = sdl2::init()?;
        let video_subsys = sdl.video()?;
        let image = image::init(InitFlag::PNG)?;
        let ttf = ttf::init().map_err(|e| e.to_string())?;
        let mut win = video_subsys
            .window("Minesweeper", 800, 600)
            .position_centered()
            .resizable()
            .build()
            .map_err(|e| e.to_string())?;
        win.set_icon(Surface::load_bmp_rw(&mut RWops::from_bytes(&resource!(
            "res/icon.png"
        ))?)?);
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
            font_res: resource!("res/font/opensans.ttf"),
        })
    }
}
