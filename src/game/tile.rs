use sdl2::{
    rect::Rect,
    render::{Texture, WindowCanvas},
};

pub const TILE_SIZE: i32 = 24;

#[derive(Clone)]
pub struct Tile {
    pub value: Option<u8>,
    pub is_mine: bool,
    pub is_flagged: bool,
    pub is_mined: bool,
    pub rect: Rect,
}

impl Tile {
    pub fn new(value: Option<u8>) -> Tile {
        Tile {
            value,
            is_mine: value.is_none(),
            is_flagged: false,
            is_mined: false,
            rect: Rect::new(0, 16, 16, 16),
        }
    }

    pub fn set_mine(&mut self) {
        self.value = None;
        self.is_mine = true;
    }

    pub fn set_value(&mut self, value: u8) {
        self.is_mine = false;
        self.value = Some(value);
    }

    pub fn mine(&mut self) -> bool {
        self.is_mined = true;
        if self.is_mine {
            self.rect.x = 32;
            true
        } else {
            self.rect.x = 16 * i32::from(self.value.unwrap());
            self.rect.y = 0;
            false
        }
    }

    pub fn flag(&mut self) -> Option<bool> {
        if self.is_mined {
            return None;
        }
        self.is_flagged = !self.is_flagged;
        if self.is_flagged {
            self.rect.x = 16;
            Some(false)
        } else {
            self.rect.x = 0;
            Some(true)
        }
    }

    pub fn render(
        &self,
        canvas: &mut WindowCanvas,
        tex: &Texture,
        x: i32,
        y: i32,
        status: Option<bool>,
    ) -> Result<(), String> {
        let mut rect = self.rect;
        if let Some(won) = status {
            if won {
                rect.y = 16;
                rect.x = if self.is_mine { 64 } else { 0 };
            } else if self.is_flagged && !self.is_mine {
                rect.x = 48;
                rect.y = 16;
            } else if !self.is_flagged && self.is_mine {
                rect.x = 32;
                rect.y = 16;
            }
        }

        canvas.copy(
            tex,
            rect,
            Rect::new(x, y, TILE_SIZE as u32, TILE_SIZE as u32),
        )
    }
}
