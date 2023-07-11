#![warn(clippy::pedantic)]

use std::time::Duration;

use context::Context;
use game::{
    map::{Map, Size},
    tile::TILE_SIZE,
    Stage,
};
use sdl2::{
    event::Event, keyboard::Scancode, mouse::MouseButton, rwops::RWops, video::WindowPos, EventPump,
};
use ui::{
    end_menu::{self, end_menu},
    main_menu::{self, main_menu},
};

mod context;
mod game;
mod stopwatch;
mod ui;

enum RunStatus {
    Menu,
    Exit,
    PlayAgain(Size),
}

fn handle_events(event_pump: &mut EventPump, map: &mut Map) -> bool {
    let shift_held = event_pump
        .keyboard_state()
        .is_scancode_pressed(Scancode::LShift)
        || event_pump
            .keyboard_state()
            .is_scancode_pressed(Scancode::RShift);

    for e in event_pump.poll_iter() {
        match e {
            Event::Quit { .. } => return true,
            Event::MouseButtonDown {
                mouse_btn, x, y, ..
            } => match mouse_btn {
                MouseButton::Left => {
                    if let Some(tile) = map.inside(x, y) {
                        if shift_held {
                            map.flag(tile);
                        } else {
                            if map.first_move {
                                map.generate_mines(&mut rand::thread_rng(), tile.0, tile.1);
                                map.generate_tiles();
                                map.stopwatch.start();
                                map.first_move = false;
                            }
                            map.mine(tile, &mut Vec::new());
                        }
                    }
                }
                MouseButton::Right => {
                    if let Some(tile) = map.inside(x, y) {
                        map.flag(tile);
                    }
                }
                _ => (),
            },
            _ => (),
        }
    }
    false
}

fn run(again: Option<Size>) -> Result<RunStatus, String> {
    let mut ctx = Context::new()?;
    let font = ctx
        .ttf
        .load_font_from_rwops(RWops::from_bytes(&ctx.font_res)?, 15)?;
    let mut map;
    if let Some(size) = again {
        map = Map::new(size, &ctx.tex_creator, &font);
    } else if let Some(status) = main_menu(
        &ctx.tex_creator,
        &ctx.ttf,
        &mut ctx.event_pump,
        &mut ctx.canvas,
        &ctx.font_res,
    )? {
        map = Map::new(
            match status {
                main_menu::ClickStatus::Small => Size::Small,
                main_menu::ClickStatus::Normal => Size::Normal,
                main_menu::ClickStatus::Large => Size::Large,
            },
            &ctx.tex_creator,
            &font,
        );
    } else {
        return Ok(RunStatus::Exit);
    }

    let win = ctx.canvas.window_mut();
    win.set_size(
        u32::try_from(map.dim.1 * TILE_SIZE).unwrap(),
        u32::try_from(map.dim.0 * TILE_SIZE).unwrap(),
    )
    .map_err(|e| e.to_string())?;
    win.set_position(WindowPos::Centered, WindowPos::Centered);

    let mut state;
    'gameloop: loop {
        if handle_events(&mut ctx.event_pump, &mut map) {
            return Ok(RunStatus::Exit);
        }

        state = map.check_state();
        match state {
            Stage::Win | Stage::Lose => break 'gameloop,
            Stage::Playing => (),
        }

        ctx.canvas.clear();
        map.render(&mut ctx.canvas, &font, &ctx.tex_creator, None)?;
        ctx.canvas.present();

        std::thread::sleep(Duration::from_nanos(1_000_000_000u64 / 60));
    }

    map.stopwatch.stop();
    if let Some(status) = end_menu(
        &state,
        &ctx.tex_creator,
        &ctx.ttf,
        &mut ctx.event_pump,
        &mut ctx.canvas,
        &ctx.font_res,
        &mut map,
    )? {
        match status {
            end_menu::ClickStatus::Menu => return Ok(RunStatus::Menu),
            end_menu::ClickStatus::Exit => return Ok(RunStatus::Exit),
            end_menu::ClickStatus::PlayAgain => return Ok(RunStatus::PlayAgain(map.size)),
        }
    }
    Ok(RunStatus::Exit)
}

fn main() -> Result<(), String> {
    let mut again_size = None;
    loop {
        match run(again_size)? {
            RunStatus::Exit => break,
            RunStatus::Menu => again_size = None,
            RunStatus::PlayAgain(size) => again_size = Some(size),
        }
    }

    Ok(())
}
