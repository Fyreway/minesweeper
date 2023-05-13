use std::time::Duration;

use context::Context;
use game::{
    map::{Map, MapSize},
    tile::TILE_SIZE,
    GameState,
};
use resource::resource;
use sdl2::{
    event::Event,
    image::LoadTexture,
    mouse::MouseButton,
    render::TextureCreator,
    rwops::RWops,
    ttf::Font,
    video::{WindowContext, WindowPos},
};
use ui::{
    end_menu::{end_menu, EndMenuClickStatus},
    main_menu::{main_menu, MainMenuClickStatus},
};

mod context;
mod game;
mod ui;

fn generate<'a>(
    size: MapSize,
    tex_creator: &'a TextureCreator<WindowContext>,
    font: &'a Font,
) -> Map<'a> {
    let mut map = Map::new(size, tex_creator, font);
    map.generate_mines(&mut rand::thread_rng());
    map.generate_tiles();
    map
}

fn run() -> Result<bool, String> {
    let mut ctx = Context::new()?;
    let res = resource!("res/font/opensans.ttf");
    let font = ctx.ttf.load_font_from_rwops(RWops::from_bytes(&res)?, 40)?;
    let mut map;
    if let Some(status) = main_menu(
        &ctx.tex_creator,
        &ctx.ttf,
        &mut ctx.event_pump,
        &mut ctx.canvas,
    )? {
        match status {
            MainMenuClickStatus::Small => {
                map = generate(MapSize::Small, &ctx.tex_creator, &font);
                let win = ctx.canvas.window_mut();
                win.set_size(9 * TILE_SIZE as u32, 9 * TILE_SIZE as u32)
                    .map_err(|e| e.to_string())?;
                win.set_position(WindowPos::Centered, WindowPos::Centered);
            }
            MainMenuClickStatus::Normal => {
                map = generate(MapSize::Normal, &ctx.tex_creator, &font);
                let win = ctx.canvas.window_mut();
                win.set_size(16 * TILE_SIZE as u32, 16 * TILE_SIZE as u32)
                    .map_err(|e| e.to_string())?;
                win.set_position(WindowPos::Centered, WindowPos::Centered);
            }
            MainMenuClickStatus::Large => {
                map = generate(MapSize::Large, &ctx.tex_creator, &font);
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
    let mut state = GameState::Playing;
    'gameloop: loop {
        for e in ctx.event_pump.poll_iter() {
            match e {
                Event::Quit { .. } => return Ok(true),
                Event::MouseButtonDown {
                    mouse_btn, x, y, ..
                } => match mouse_btn {
                    MouseButton::Left => {
                        if let Some(tile) = map.inside(x, y) {
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
            GameState::Win | GameState::Lose => break 'gameloop,
            GameState::Playing => (),
        }

        ctx.canvas.clear();
        map.render(&mut ctx.canvas, &tex, &font, &ctx.tex_creator)?;
        ctx.canvas.present();

        std::thread::sleep(Duration::from_nanos(1_000_000_000u64 / 60));
    }

    if let Some(status) = end_menu(
        state,
        &ctx.tex_creator,
        &ctx.ttf,
        &mut ctx.event_pump,
        &mut ctx.canvas,
    )? {
        match status {
            EndMenuClickStatus::Continue => return Ok(false),
            EndMenuClickStatus::Exit => return Ok(true),
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
