use sdl2::{
    mouse::MouseState,
    render::{TextureCreator, WindowCanvas},
    ttf::Font,
    video::WindowContext,
};

use super::{button::Button, text::Text};

#[macro_export]
macro_rules! buttons {
    [{scale: $scale:expr, path: $path:expr, tex_creator: $tex_creator:expr, font: $font:expr}: $( ($x:expr, $y:expr, $w:expr) : $text:expr ),*] => {
        {
            vec![
                $(
                    Button::new($x, $y, $w, $scale, $tex_creator, std::path::Path::new($path), $text, $font)
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
    _click_handler: C,
}

impl<'a, C: ClickHandler> Menu<'a, C> {
    pub fn new(btns: Vec<Button<'a>>, texts: Vec<Text<'a>>) -> Menu<'a, C> {
        Menu {
            btns,
            texts,
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
