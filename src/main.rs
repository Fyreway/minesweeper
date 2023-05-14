#![warn(clippy::pedantic)]

use std::time::Duration;

use context::Context;
use game::{
    map::{Map, Size},
    tile::TILE_SIZE,
    Stage,
};
use resource::resource;
use sdl2::{event::Event, image::LoadTexture, mouse::MouseButton, rwops::RWops, video::WindowPos};
use ui::{
    end_menu::{self, end_menu},
    main_menu::{self, main_menu},
};

mod context;
mod game;
mod ui;

fn run() -> Result<bool, String> {
    let mut ctx = Context::new()?;
    let font = ctx
        .ttf
        .load_font_from_rwops(RWops::from_bytes(&ctx.font_res)?, 15)?;
    let mut map;
    if let Some(status) = main_menu(
        &ctx.tex_creator,
        &ctx.ttf,
        &mut ctx.event_pump,
        &mut ctx.canvas,
        &ctx.font_res,
    )? {
        match status {
            main_menu::ClickStatus::Small => {
                map = Map::new(&Size::Small, &ctx.tex_creator, &font);
                let win = ctx.canvas.window_mut();
                win.set_size(9 * TILE_SIZE as u32, 9 * TILE_SIZE as u32)
                    .map_err(|e| e.to_string())?;
                win.set_position(WindowPos::Centered, WindowPos::Centered);
            }
            main_menu::ClickStatus::Normal => {
                map = Map::new(&Size::Normal, &ctx.tex_creator, &font);
                let win = ctx.canvas.window_mut();
                win.set_size(16 * TILE_SIZE as u32, 16 * TILE_SIZE as u32)
                    .map_err(|e| e.to_string())?;
                win.set_position(WindowPos::Centered, WindowPos::Centered);
            }
            main_menu::ClickStatus::Large => {
                map = Map::new(&Size::Large, &ctx.tex_creator, &font);
                let win = ctx.canvas.window_mut();
                win.set_size(30 * TILE_SIZE as u32, 18 * TILE_SIZE as u32)
                    .map_err(|e| e.to_string())?;
                win.set_position(WindowPos::Centered, WindowPos::Centered);
            }
        }
    } else {
        return Ok(true);
    }

    let tex = ctx
        .tex_creator
        .load_texture_bytes(&resource!("res/spritesheet.png"))?;
    let mut state = Stage::Playing;
    'gameloop: loop {
        for e in ctx.event_pump.poll_iter() {
            match e {
                Event::Quit { .. } => return Ok(true),
                Event::MouseButtonDown {
                    mouse_btn, x, y, ..
                } => match mouse_btn {
                    MouseButton::Left => {
                        if let Some(tile) = map.inside(x, y) {
                            if map.first_move {
                                map.generate_mines(&mut rand::thread_rng(), tile.0, tile.1);
                                map.generate_tiles();
                                map.stopwatch.start();
                                map.first_move = false;
                            }
                            map.mine(tile, &mut Vec::new());
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
            if let Event::Quit { .. } = e {
                break 'gameloop;
            }
        }

        state = map.check_state();
        match state {
            Stage::Win | Stage::Lose => break 'gameloop,
            Stage::Playing => (),
        }

        ctx.canvas.clear();
        map.render(&mut ctx.canvas, &tex, &font, &ctx.tex_creator)?;
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
        map.stopwatch.elapsed().as_secs(),
    )? {
        match status {
            end_menu::ClickStatus::Menu => return Ok(false),
            end_menu::ClickStatus::Exit => return Ok(true),
        }
    }
    Ok(false)
}

fn main() -> Result<(), String> {
    loop {
        if run()? {
            break;
        }
    }

    Ok(())
}
