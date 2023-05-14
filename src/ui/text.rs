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
    #[allow(clippy::too_many_arguments)]
    pub fn new<'b>(
        x: i32,
        y: i32,
        offset_x: i32,
        offset_y: i32,
        tex_creator: &'b TextureCreator<WindowContext>,
        text: &str,
        font: &Font,
        width: u32,
        height: u32,
    ) -> Text<'b> {
        let text_surf = font
            .render(text)
            .blended(Color::WHITE)
            .expect("Could not get text surface");
        let text_tex = tex_creator
            .create_texture_from_surface(&text_surf)
            .expect("Could not get text texture");
        let TextureQuery {
            width: t_width,
            height: t_height,
            ..
        } = text_tex.query();
        let x_ = if x == POS_CENTERED {
            i32::try_from((width - t_width) / 2).unwrap()
        } else {
            x
        } + offset_x;
        let y_ = if y == POS_CENTERED {
            i32::try_from((height - t_height) / 2).unwrap()
        } else {
            y
        } + offset_y;

        Text {
            text: text.to_string(),
            prev_text: text.to_string(),
            text_tex,
            text_rect: Rect::new(x_, y_ + 16 / 4, t_width, t_height),
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
                .blended(Color::WHITE)
                .expect("Could not get text surface");
            self.text_tex = tex_creator
                .create_texture_from_surface(&text_surf)
                .expect("Could not get text texture");
            self.prev_text = self.text.clone();
        }
        canvas.copy(&self.text_tex, None, self.text_rect)
    }
}
