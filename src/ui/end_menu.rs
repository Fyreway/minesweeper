use std::time::Duration;

use resource::Resource;

use sdl2::{
    event::Event,
    mouse::MouseButton,
    pixels::Color,
    rect::Rect,
    render::{TextureCreator, WindowCanvas},
    rwops::RWops,
    ttf::Sdl2TtfContext,
    video::WindowContext,
    EventPump,
};

use crate::{
    buttons,
    game::{map::Map, tile::TILE_SIZE, Stage},
    texts,
};

use super::{
    button::Button,
    menu::{ClickHandler, Menu},
    text::Text,
    POS_CENTERED,
};

pub enum ClickStatus {
    Menu,
    Exit,
    PlayAgain,
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
        } else if btns[2].inside(x, y) {
            Some(ClickStatus::PlayAgain)
        } else {
            None
        }
    }
}

pub fn end_menu<'a>(
    state: &Stage,
    tex_creator: &'a TextureCreator<WindowContext>,
    ttf: &Sdl2TtfContext,
    event_pump: &mut EventPump,
    canvas: &mut WindowCanvas,
    font_res: &Resource<[u8]>,
    map: &mut Map<'a>,
) -> Result<Option<ClickStatus>, String> {
    let font = ttf.load_font_from_rwops(RWops::from_bytes(font_res)?, 40)?;
    let map_width = map.dim.1 * TILE_SIZE;
    let map_height = map.dim.0 * TILE_SIZE;
    let mut end_menu = Menu::<EndMenuHandler>::new(
        buttons![
            { 5, tex_creator, ttf, font_res, map_width, 0, 400, 500 }:
            (POS_CENTERED, 300, 64, 7) : "Menu",
            (POS_CENTERED, 400, 64, 7) : "Exit",
            (POS_CENTERED, 500, 64, 5) : &format!("Play {} Again", map.size.to_string())
        ],
        texts![
            { tex_creator, ttf, font_res, map_width, 0, 400, 500 }:
            (POS_CENTERED, 50, 50) : if let Stage::Lose = state {"You Lose!"} else {"You Win!"},
            (POS_CENTERED, 200, 20) : &format!("Time: {}", map.stopwatch.elapsed().as_secs()),
            (5, 610, 20) : &format!("minesweeper v{}", env!("CARGO_PKG_VERSION"))
        ],
        (400 + u32::try_from(map_width).unwrap(), 650),
    );

    loop {
        for e in event_pump.poll_iter() {
            match e {
                Event::Quit { .. } => return Ok(None),
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

        canvas.clear();

        canvas.set_draw_color(Color::RGB(100, 100, 100));
        canvas.fill_rect(Rect::new(
            0,
            0,
            u32::try_from(map_width + 5).unwrap(),
            u32::try_from(map_height + 5).unwrap(),
        ))?;
        canvas.set_draw_color(Color::RGB(28, 28, 28));

        map.render(canvas, &font, tex_creator, Some(state == &Stage::Win))?;

        end_menu.render(canvas, &font, tex_creator)?;
        std::thread::sleep(Duration::from_nanos(1_000_000_000u64 / 60));
    }
}
