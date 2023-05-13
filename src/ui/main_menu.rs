use sdl2::{
    event::Event,
    mouse::MouseState,
    render::{TextureCreator, WindowCanvas},
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

pub enum MainMenuClickStatus {
    Small,
    Normal,
    Large,
}

#[derive(Default)]
struct MainMenuHandler {}

impl ClickHandler for MainMenuHandler {
    type Type = MainMenuClickStatus;

    fn handle_clicks(btns: &[Button<'_>], m: &MouseState) -> Option<Self::Type> {
        if btns[0].inside(m) {
            Some(MainMenuClickStatus::Small)
        } else if btns[1].inside(m) {
            Some(MainMenuClickStatus::Normal)
        } else if btns[2].inside(m) {
            Some(MainMenuClickStatus::Large)
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
) -> Result<Option<MainMenuClickStatus>, String> {
    // let main_menu = MainMenu::new(&ctx.ttf, &ctx.tex_creator);
    let small_font = ttf.load_font("res/font/opensans.ttf", 40)?;
    let mut title_font = ttf.load_font("res/font/opensans.ttf", 500)?;
    title_font.set_style(FontStyle::BOLD);
    let mut main_menu = Menu::<MainMenuHandler>::new(
        buttons![
            {
                scale: 5,
                path: "res/button.png",
                tex_creator: tex_creator,
                font: &small_font
            }:
            (POS_CENTERED, 300, 64) : "Small",
            (POS_CENTERED, 400, 64) : "Normal",
            (POS_CENTERED, 500, 64) : "Large"
        ],
        texts![
            {
                scale: 15,
                tex_creator: tex_creator,
                font: &title_font
            }:
            (POS_CENTERED, 50) : "MINESWEEPER"
        ],
    );

    'top: loop {
        for e in event_pump.poll_iter() {
            if let Event::Quit { .. } = e {
                break 'top;
            }
        }
        let mouse_state = event_pump.mouse_state();
        if mouse_state.left() {
            if let Some(status) = main_menu.handle_clicks(&mouse_state) {
                return Ok(Some(status));
            }
        }

        main_menu.render(canvas, &small_font, tex_creator)?;
        std::thread::sleep(Duration::from_nanos(1_000_000_000u64 / 60));
    }

    Ok(None)
}
