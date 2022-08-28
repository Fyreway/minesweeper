use std::path::PathBuf;

use sdl2::{
    image::LoadTexture,
    rect::Rect,
    render::{TextureCreator, WindowCanvas},
};

pub struct Button<'a> {
    x: i32,
    y: i32,
    w: u32,
    scale: u32,
    img: sdl2::render::Texture<'a>,
}
impl Button<'_> {
    pub fn new<'a, T: 'a>(
        x: i32,
        y: i32,
        w: u32,
        scale: u32,
        tex_creator: &'a mut TextureCreator<T>,
        file: PathBuf,
    ) -> Button<'a> {
        let x_ = if x == -1 {
            ((800 - w * scale) / 2) as _
        } else {
            x
        };
        let y_ = if y == -1 {
            ((600 - 16 * scale) / 2) as _
        } else {
            y
        };
        Button {
            x: x_,
            y: y_,
            w,
            scale,
            img: tex_creator
                .load_texture(file)
                .expect("Could not load image"),
        }
    }

    pub fn render(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        canvas.copy(
            &self.img,
            Rect::new(0, 0, 16, 16),
            Rect::new(self.x, self.y, 16 * self.scale, 16 * self.scale),
        )?;

        let middle_width = self.w - 32;

        if middle_width > 0 {
            canvas.copy(
                &self.img,
                Rect::new(16, 0, 16, 16),
                Rect::new(
                    self.x + 16 * self.scale as i32,
                    self.y,
                    middle_width * self.scale,
                    16 * self.scale,
                ),
            )?;
        }

        canvas.copy(
            &self.img,
            Rect::new(32, 0, 16, 16),
            Rect::new(
                self.x + ((16 + middle_width) * self.scale) as i32,
                self.y,
                16 * self.scale,
                16 * self.scale,
            ),
        )
    }
}
