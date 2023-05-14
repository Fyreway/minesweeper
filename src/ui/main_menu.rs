use resource::{resource, Resource};

use sdl2::{
    event::Event,
    mouse::MouseButton,
    pixels::Color,
    render::{TextureCreator, WindowCanvas},
    rwops::RWops,
    ttf::Sdl2TtfContext,
    video::WindowContext,
    EventPump,
};
use std::time::Duration;

use crate::{
    buttons, texts,
    ui::{button::Button, text::Text, POS_CENTERED},
};

use super::menu::{ClickHandler, Menu};

pub enum ClickStatus {
    Small,
    Normal,
    Large,
    // Custom,
}

#[derive(Default)]
struct MainMenuHandler {}

impl ClickHandler for MainMenuHandler {
    type Type = ClickStatus;

    fn handle_clicks(btns: &[Button<'_>], x: i32, y: i32) -> Option<Self::Type> {
        if btns[0].inside(x, y) {
            Some(ClickStatus::Small)
        } else if btns[1].inside(x, y) {
            Some(ClickStatus::Normal)
        } else if btns[2].inside(x, y) {
            Some(ClickStatus::Large)
        // } else if btns[3].inside(m) {
        //     Some(MainMenuClickStatus::Custom)
        } else {
            None
        }
    }
}

pub fn main_menu(
    tex_creator: &TextureCreator<WindowContext>,
    ttf: &Sdl2TtfContext,
    event_pump: &mut EventPump,
    canvas: &mut WindowCanvas,
    font_res: &Resource<[u8]>,
) -> Result<Option<ClickStatus>, String> {
    let res = resource!("res/font/opensans.ttf");
    let font = ttf.load_font_from_rwops(RWops::from_bytes(&res)?, 50)?;
    let mut main_menu = Menu::<MainMenuHandler>::new(
        buttons![
            { 5, tex_creator, ttf, font_res, 800, 600 }:
            (POS_CENTERED, 300, 64, 7) : "Small",
            (POS_CENTERED, 400, 64, 7) : "Normal",
            (POS_CENTERED, 500, 64, 7) : "Large"
            // (POS_CENTERED, 600, 64) : "Custom"
        ],
        texts![
            { tex_creator, ttf, font_res, 800, 600 }:
            (POS_CENTERED, 50, 90) : "MINESWEEPER",
            (5, 560, 20) : &format!("minesweeper v{}", env!("CARGO_PKG_VERSION"))
        ],
        (800, 600),
    );

    'top: loop {
        for e in event_pump.poll_iter() {
            match e {
                Event::Quit { .. } => break 'top,
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    x,
                    y,
                    ..
                } => {
                    if let Some(status) = main_menu.handle_clicks(x, y) {
                        return Ok(Some(status));
                    }
                }
                _ => (),
            }
        }

        canvas.set_draw_color(Color::RGB(28, 28, 28));
        canvas.clear();
        main_menu.render(canvas, &font, tex_creator)?;
        std::thread::sleep(Duration::from_nanos(1_000_000_000u64 / 60));
    }

    Ok(None)
}
