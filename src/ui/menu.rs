use sdl2::{
    render::{TextureCreator, WindowCanvas},
    ttf::Font,
    video::{WindowContext, WindowPos},
};

use super::{button::Button, text::Text};

#[macro_export]
macro_rules! buttons {
    [{$scale:expr, $tex_creator:expr, $ttf:expr, $res:expr}: $( ($x:expr, $y:expr, $w:expr, $size:expr) : $text:expr ),*] => {
        {
            vec![
                $(
                    Button::new($x, $y, $w, $scale, $tex_creator, $text, &$ttf.load_font_from_rwops(RWops::from_bytes(&$res)?, $size * $scale)?)
                ),*
            ]
        }
    };
}

#[macro_export]
macro_rules! texts {
    [{$tex_creator:expr, $ttf:expr, $res:expr}: $( ($x:expr, $y:expr, $size:expr) : $text:expr ),*] => {
        {
            vec![
                $(
                    Text::new($x, $y, $tex_creator, $text, &$ttf.load_font_from_rwops(RWops::from_bytes(&$res)?, $size)?)
                ),*
            ]
        }
    }
}

pub trait ClickHandler: Default {
    type Type;
    fn handle_clicks(btns: &[Button<'_>], x: i32, y: i32) -> Option<Self::Type>;
}

pub struct Menu<'a, C: ClickHandler> {
    btns: Vec<Button<'a>>,
    texts: Vec<Text<'a>>,
    size: (u32, u32),
    _click_handler: C,
}

impl<'a, C: ClickHandler> Menu<'a, C> {
    pub fn new(btns: Vec<Button<'a>>, texts: Vec<Text<'a>>, size: (u32, u32)) -> Menu<'a, C> {
        Menu {
            btns,
            texts,
            size,
            _click_handler: C::default(),
        }
    }

    pub fn handle_clicks(&self, x: i32, y: i32) -> Option<C::Type> {
        C::handle_clicks(&self.btns, x, y)
    }

    pub fn render(
        &mut self,
        canvas: &mut WindowCanvas,
        font: &Font,
        tex_creator: &'a TextureCreator<WindowContext>,
    ) -> Result<(), String> {
        canvas.clear();

        if canvas.window().size() != self.size {
            let win = canvas.window_mut();
            win.set_size(self.size.0, self.size.1)
                .map_err(|e| e.to_string())?;
            win.set_position(WindowPos::Centered, WindowPos::Centered);
        }

        for btn in &self.btns {
            btn.render(canvas)?;
        }

        for text in &mut self.texts {
            text.render(canvas, font, tex_creator)?;
        }

        canvas.present();
        Ok(())
    }
}
