use sdl2::{
    event::Event, mouse::MouseState, render::TextureCreator, render::WindowCanvas,
    ttf::Sdl2TtfContext, video::WindowContext,
};
use std::{path::Path, time::Duration};

use crate::{button::Button, Context};

struct MainMenu<'a> {
    btns: Vec<Button<'a>>,
}
impl MainMenu<'_> {
    fn new<'a>(
        ttf: &Sdl2TtfContext,
        tex_creator: &'a TextureCreator<WindowContext>,
    ) -> MainMenu<'a> {
        MainMenu {
            btns: vec![
                Button::new(
                    -1,
                    300,
                    64,
                    5,
                    tex_creator,
                    &Path::new("res/button.png"),
                    "Small".to_string(),
                    &ttf.load_font("res/font/opensans.ttf", 40)
                        .expect("Could not load font"),
                ),
                Button::new(
                    -1,
                    400,
                    64,
                    5,
                    tex_creator,
                    &Path::new("res/button.png"),
                    "Normal".to_string(),
                    &ttf.load_font("res/font/opensans.ttf", 40)
                        .expect("Could not load font"),
                ),
                Button::new(
                    -1,
                    500,
                    64,
                    5,
                    tex_creator,
                    &Path::new("res/button.png"),
                    "Large".to_string(),
                    &ttf.load_font("res/font/opensans.ttf", 40)
                        .expect("Could not load font"),
                ),
            ],
        }
    }

    fn handle_clicks(&self, m: &MouseState) {
        if self.btns[0].is_clicked(m) {
            println!("Small button clicked");
        } else if self.btns[1].is_clicked(m) {
            println!("Normal button clicked");
        } else if self.btns[2].is_clicked(m) {
            println!("Large button clicked");
        }
    }

    fn render(&self, canvas: &mut WindowCanvas) {
        canvas.clear();

        for btn in self.btns.as_slice() {
            btn.render(canvas).expect("Could not render button");
        }

        canvas.present();
    }
}

pub fn main_menu(ctx: &mut Context) {
    let main_menu = MainMenu::new(&ctx.ttf, &ctx.tex_creator);

    'top: loop {
        for e in ctx.event_pump.poll_iter() {
            match e {
                Event::Quit { .. } => break 'top,
                _ => (),
            }
        }
        let mouse_state = ctx.event_pump.mouse_state();
        if mouse_state.left() {
            main_menu.handle_clicks(&mouse_state);
        }

        main_menu.render(&mut ctx.canvas);
        std::thread::sleep(Duration::from_nanos(1_000_000_000u64 / 60));
    }
}
