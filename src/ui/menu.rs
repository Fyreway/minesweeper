use sdl2::{
    mouse::MouseState,
    render::{TextureCreator, WindowCanvas},
    ttf::Font,
    video::{WindowContext, WindowPos},
};

use super::{button::Button, text::Text};

#[macro_export]
macro_rules! buttons {
    [{scale: $scale:expr, tex_creator: $tex_creator:expr, font: $font:expr}: $( ($x:expr, $y:expr, $w:expr) : $text:expr ),*] => {
        {
            vec![
                $(
                    Button::new($x, $y, $w, $scale, $tex_creator, $text, $font)
                ),*
            ]
        }
    };
}

#[macro_export]
macro_rules! texts {
    [{scale: $scale:expr, tex_creator: $tex_creator:expr, font: $font:expr}: $( ($x:expr, $y:expr) : $text:expr ),*] => {
        {
            vec![
                $(
                    Text::new($x, $y, $scale, $tex_creator, $text, $font)
                ),*
            ]
        }
    }
}

pub trait ClickHandler: Default {
    type Type;
    fn handle_clicks(btns: &[Button<'_>], m: &MouseState) -> Option<Self::Type>;
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

    pub fn handle_clicks(&self, m: &MouseState) -> Option<C::Type> {
        C::handle_clicks(&self.btns, m)
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
