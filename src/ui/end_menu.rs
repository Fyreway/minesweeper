use std::time::Duration;

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

use crate::{buttons, game::Stage, texts};

use super::{
    button::Button,
    menu::{ClickHandler, Menu},
    text::Text,
    POS_CENTERED,
};

pub enum ClickStatus {
    Menu,
    Exit,
}

#[derive(Default)]
struct EndMenuHandler {}

impl ClickHandler for EndMenuHandler {
    type Type = ClickStatus;

    fn handle_clicks(btns: &[Button<'_>], x: i32, y: i32) -> Option<Self::Type> {
        if btns[0].inside(x, y) {
            Some(ClickStatus::Menu)
        } else if btns[1].inside(x, y) {
            Some(ClickStatus::Exit)
        } else {
            None
        }
    }
}

pub fn end_menu(
    state: &Stage,
    tex_creator: &TextureCreator<WindowContext>,
    ttf: &Sdl2TtfContext,
    event_pump: &mut EventPump,
    canvas: &mut WindowCanvas,
) -> Result<Option<ClickStatus>, String> {
    let res = resource!("res/font/opensans.ttf");
    let small_font = ttf.load_font_from_rwops(RWops::from_bytes(&res)?, 40)?;
    let mut title_font = ttf.load_font_from_rwops(RWops::from_bytes(&res)?, 500)?;
    title_font.set_style(FontStyle::BOLD);
    let mut end_menu = Menu::<EndMenuHandler>::new(
        buttons![
            {
                scale: 5,
                tex_creator: tex_creator,
                font: &small_font
            }:
            (POS_CENTERED, 300, 64) : "Menu",
            (POS_CENTERED, 400, 64) : "Exit"
        ],
        texts![
            {
                tex_creator: tex_creator,
                font: &title_font
            }:
            (POS_CENTERED, 50, 15) : if let Stage::Lose = state {"You Lose!"} else {"You Win!"}
        ],
        (800, 500),
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
                    if let Some(status) = end_menu.handle_clicks(x, y) {
                        return Ok(Some(status));
                    }
                }
                _ => (),
            }
        }

        end_menu.render(canvas, &small_font, tex_creator)?;
        std::thread::sleep(Duration::from_nanos(1_000_000_000u64 / 60));
    }

    Ok(None)
}
