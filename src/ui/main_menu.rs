use resource::resource;

use sdl2::{
    event::Event,
    mouse::MouseButton,
    render::{TextureCreator, WindowCanvas},
    rwops::RWops,
    ttf::{FontStyle, Sdl2TtfContext},
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
) -> Result<Option<ClickStatus>, String> {
    let res = resource!("res/font/opensans.ttf");
    let small_font = ttf.load_font_from_rwops(RWops::from_bytes(&res)?, 40)?;
    let mut title_font = ttf.load_font_from_rwops(RWops::from_bytes(&res)?, 500)?;
    title_font.set_style(FontStyle::BOLD);
    let mut main_menu = Menu::<MainMenuHandler>::new(
        buttons![
            {
                scale: 5,
                tex_creator: tex_creator,
                font: &small_font
            }:
            (POS_CENTERED, 300, 64) : "Small",
            (POS_CENTERED, 400, 64) : "Normal",
            (POS_CENTERED, 500, 64) : "Large"
            // (POS_CENTERED, 600, 64) : "Custom"
        ],
        texts![
            {
                scale: 15,
                tex_creator: tex_creator,
                font: &title_font
            }:
            (POS_CENTERED, 50) : "MINESWEEPER"
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

        main_menu.render(canvas, &small_font, tex_creator)?;
        std::thread::sleep(Duration::from_nanos(1_000_000_000u64 / 60));
    }

    Ok(None)
}
