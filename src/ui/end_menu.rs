use std::time::Duration;

use sdl2::{
    event::Event,
    mouse::MouseState,
    render::{TextureCreator, WindowCanvas},
    ttf::{FontStyle, Sdl2TtfContext},
    video::WindowContext,
    EventPump,
};

use crate::{buttons, game::GameState, texts};

use super::{
    button::Button,
    menu::{ClickHandler, Menu},
    text::Text,
    POS_CENTERED,
};

pub enum EndMenuClickStatus {
    Continue,
    Exit,
}

#[derive(Default)]
struct EndMenuHandler {}

impl ClickHandler for EndMenuHandler {
    type Type = EndMenuClickStatus;

    fn handle_clicks(btns: &[Button<'_>], m: &MouseState) -> Option<Self::Type> {
        if btns[0].inside(m) {
            Some(EndMenuClickStatus::Continue)
        } else if btns[1].inside(m) {
            Some(EndMenuClickStatus::Exit)
        } else {
            None
        }
    }
}

pub fn end_menu(
    state: GameState,
    tex_creator: &TextureCreator<WindowContext>,
    ttf: &Sdl2TtfContext,
    event_pump: &mut EventPump,
    canvas: &mut WindowCanvas,
) -> Result<Option<EndMenuClickStatus>, String> {
    let small_font = ttf.load_font("res/font/opensans.ttf", 40)?;
    let mut title_font = ttf.load_font("res/font/opensans.ttf", 500)?;
    title_font.set_style(FontStyle::BOLD);
    let mut main_menu = Menu::<EndMenuHandler>::new(
        buttons![
            {
                scale: 5,
                path: "res/button.png",
                tex_creator: tex_creator,
                font: &small_font
            }:
            (POS_CENTERED, 300, 64) : "Continue",
            (POS_CENTERED, 400, 64) : "Exit"
        ],
        texts![
            {
                scale: 15,
                tex_creator: tex_creator,
                font: &title_font
            }:
            (POS_CENTERED, 50) : if let GameState::Lose = state {"You Lose!"} else {"You Win!"}
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
