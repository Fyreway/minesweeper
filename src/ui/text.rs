use sdl2::{
    pixels::Color,
    rect::Rect,
    render::{Texture, TextureCreator, TextureQuery, WindowCanvas},
    ttf::Font,
    video::WindowContext,
};

use super::POS_CENTERED;

pub struct Text<'a> {
    text: String,
    prev_text: String,
    text_tex: Texture<'a>,
    text_rect: Rect,
}

impl<'a> Text<'a> {
    pub fn new<'b>(
        x: i32,
        y: i32,
        tex_creator: &'b TextureCreator<WindowContext>,
        text: &str,
        font: &Font,
    ) -> Text<'b> {
        let text_surf = font
            .render(text)
            .blended(Color::WHITE)
            .expect("Could not get text surface");
        let text_tex = tex_creator
            .create_texture_from_surface(&text_surf)
            .expect("Could not get text texture");
        let TextureQuery { width, height, .. } = text_tex.query();
        let x_ = if x == POS_CENTERED {
            i32::try_from((800 - width) / 2).unwrap()
        } else {
            x
        };
        let y_ = if y == POS_CENTERED {
            i32::try_from((600 - height) / 2).unwrap()
        } else {
            y
        };

        Text {
            text: text.to_string(),
            prev_text: text.to_string(),
            text_tex,
            text_rect: Rect::new(x_, y_ + 16 / 4, width, height),
        }
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }

    pub fn render(
        &mut self,
        canvas: &mut WindowCanvas,
        font: &Font,
        tex_creator: &'a TextureCreator<WindowContext>,
    ) -> Result<(), String> {
        if self.prev_text != self.text {
            let text_surf = font
                .render(&self.text)
                .solid(Color::WHITE)
                .expect("Could not get text surface");
            self.text_tex = tex_creator
                .create_texture_from_surface(&text_surf)
                .expect("Could not get text texture");
            self.prev_text = self.text.clone();
        }
        canvas.copy(&self.text_tex, None, self.text_rect)
    }
}
