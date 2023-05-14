use resource::resource;
use sdl2::{
    image::LoadTexture,
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
        offset_x: i32,
        offset_y: i32,
        w: i32,
        scale: i32,
        tex_creator: &'a TextureCreator<WindowContext>,
        text: &str,
        font: &Font,
        width: i32,
        height: i32,
    ) -> Button<'a> {
        let x_ = if x == POS_CENTERED {
            ((width - w * scale) / 2) as _
        } else {
            x
        } + offset_x;
        let y_ = if y == POS_CENTERED {
            ((height - 16 * scale) / 2) as _
        } else {
            y
        } + offset_y;
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
        let text_width = i32::try_from(t_width).unwrap();
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
                t_width,
                t_height,
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
                u32::try_from(16 * self.scale).unwrap(),
                u32::try_from(16 * self.scale).unwrap(),
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
                    u32::try_from(middle_width * self.scale).unwrap(),
                    u32::try_from(16 * self.scale).unwrap(),
                ),
            )?;
        }

        canvas.copy(
            &self.img,
            Rect::new(32, 0, 16, 16),
            Rect::new(
                self.x + (16 + middle_width) * self.scale,
                self.y,
                u32::try_from(16 * self.scale).unwrap(),
                u32::try_from(16 * self.scale).unwrap(),
            ),
        )?;

        canvas.copy(&self.text_tex, None, self.text_rect)
    }

    pub fn inside(&self, x: i32, y: i32) -> bool {
        x > self.x && x < self.x + self.w * self.scale && y > self.y && y < self.y + 16 * self.scale
    }
}
