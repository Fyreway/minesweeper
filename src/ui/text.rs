use sdl2::{
    pixels::Color,
    rect::Rect,
    render::{Texture, TextureCreator, TextureQuery, WindowCanvas},
    ttf::Font,
    video::WindowContext,
};

use super::POS_CENTERED;

pub struct Text<'a> {
    x: i32,
    y: i32,
    scale: i32,
    text: String,
    prev_text: String,
    text_tex: Texture<'a>,
    text_rect: Rect,
}

impl<'a> Text<'a> {
    pub fn new<'b>(
        x: i32,
        y: i32,
        scale: i32,
        tex_creator: &'b TextureCreator<WindowContext>,
        text: &str,
        font: &Font,
    ) -> Text<'b> {
        let text_surf = font
            .render(text)
            .solid(Color::WHITE)
            .expect("Could not get text surface");
        let text_tex = tex_creator
            .create_texture_from_surface(&text_surf)
            .expect("Could not get text texture");
        let TextureQuery { width, height, .. } = text_tex.query();
        let text_width = width as i32 * 8 * scale / height as i32;
        let x_ = if x == POS_CENTERED {
            ((800 - text_width) / 2) as _
        } else {
            x
        };
        let y_ = if y == POS_CENTERED {
            ((600 - 16 * scale) / 2) as _
        } else {
            y
        };

        Text {
            x: x_,
            y: y_,
            scale,
            text: text.to_string(),
            prev_text: text.to_string(),
            text_tex,
            text_rect: Rect::new(
                x_,
                y_ + (16 * scale) / 4,
                text_width as _,
                ((16 * scale) / 2) as _,
            ),
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
