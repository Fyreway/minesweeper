use resource::resource;
use sdl2::{
    image::LoadTexture,
    mouse::MouseState,
    pixels::Color,
    rect::Rect,
    render::{Texture, TextureCreator, TextureQuery, WindowCanvas},
    ttf::Font,
    video::WindowContext,
};

use super::POS_CENTERED;

pub struct Button<'a> {
    x: i32,
    y: i32,
    w: i32,
    scale: i32,
    img: Texture<'a>,
    text_tex: Texture<'a>,
    text_rect: Rect,
}

impl Button<'_> {
    #[allow(clippy::too_many_arguments)]
    pub fn new<'a>(
        x: i32,
        y: i32,
        w: i32,
        scale: i32,
        tex_creator: &'a TextureCreator<WindowContext>,
        text: &str,
        font: &Font,
    ) -> Button<'a> {
        let x_ = if x == POS_CENTERED {
            ((800 - w * scale) / 2) as _
        } else {
            x
        };
        let y_ = if y == POS_CENTERED {
            ((600 - 16 * scale) / 2) as _
        } else {
            y
        };
        let text_surf = font
            .render(text)
            .solid(Color::WHITE)
            .expect("Could not get text surface");
        let text_tex = tex_creator
            .create_texture_from_surface(&text_surf)
            .expect("Could not get text texture");
        let TextureQuery { width, height, .. } = text_tex.query();
        let text_width = width as i32 * 8 * scale / height as i32;
        Button {
            x: x_,
            y: y_,
            w,
            scale,
            img: tex_creator
                .load_texture_bytes(&resource!("res/button.png"))
                .expect("Could not load image"),
            text_tex,
            text_rect: Rect::new(
                x_ + (w * scale - text_width) / 2,
                y_ + (16 * scale) / 4,
                text_width as _,
                ((16 * scale) / 2) as _,
            ),
        }
    }

    pub fn render(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        canvas.copy(
            &self.img,
            Rect::new(0, 0, 16, 16),
            Rect::new(
                self.x,
                self.y,
                (16 * self.scale) as _,
                (16 * self.scale) as _,
            ),
        )?;

        let middle_width = self.w - 32;

        if middle_width > 0 {
            canvas.copy(
                &self.img,
                Rect::new(16, 0, 16, 16),
                Rect::new(
                    self.x + 16 * self.scale,
                    self.y,
                    (middle_width * self.scale) as _,
                    (16 * self.scale) as _,
                ),
            )?;
        }

        canvas.copy(
            &self.img,
            Rect::new(32, 0, 16, 16),
            Rect::new(
                self.x + (16 + middle_width) * self.scale,
                self.y,
                (16 * self.scale) as _,
                (16 * self.scale) as _,
            ),
        )?;

        canvas.copy(&self.text_tex, None, self.text_rect)
    }

    pub fn inside(&self, m: &MouseState) -> bool {
        m.x() > self.x
            && m.x() < self.x + self.w * self.scale
            && m.y() > self.y
            && m.y() < self.y + 16 * self.scale
    }
}
