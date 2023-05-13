use rand::{rngs::ThreadRng, Rng};
use sdl2::{
    render::{Texture, TextureCreator, WindowCanvas},
    ttf::Font,
    video::WindowContext,
};

use crate::ui::text::Text;

use super::{
    tile::{Tile, TILE_SIZE},
    Coords, GameState,
};

pub enum MapSize {
    Small,
    Normal,
    Large,
}

pub struct Map<'a> {
    size: MapSize,
    dim: Coords<usize>,
    map: Vec<Vec<Tile>>,
    lost: bool,
    mines: u8,
    flags: u8,
    flags_text: Text<'a>,
}

impl<'a> Map<'a> {
    pub fn new(
        size: MapSize,
        tex_creator: &'a TextureCreator<WindowContext>,
        font: &'a Font,
    ) -> Map<'a> {
        let dim = match size {
            MapSize::Small => (9, 9),
            MapSize::Normal => (16, 16),
            MapSize::Large => (18, 30),
        };
        let mines = match size {
            MapSize::Small => 10,
            MapSize::Normal => 40,
            MapSize::Large => 99,
        };
        Map {
            size,
            dim,
            map: {
                let mut map = vec![];
                for _ in 0..dim.1 {
                    let mut row = vec![];
                    for _ in 0..dim.0 {
                        row.push(Tile::new(Some(0)));
                    }
                    map.push(row);
                }
                map
            },
            lost: false,
            mines,
            flags: mines,
            flags_text: Text::new(0, -10, 3, tex_creator, &format!("Flags: {mines}"), font),
        }
    }

    fn get(&self, pos: Coords<usize>) -> Option<&Tile> {
        if pos.0 > self.dim.0 - 1 || pos.1 > self.dim.1 - 1 {
            None
        } else {
            Some(&self.map[pos.1][pos.0])
        }
    }

    fn get_adjacent_tiles(&self, pos: Coords<usize>) -> Vec<Coords<usize>> {
        let mut adjacent = vec![];
        let (col, row) = pos;
        if col > 0 {
            if row > 0 {
                adjacent.push((col - 1, row - 1));
            }
            adjacent.push((col - 1, row));
            if row < self.dim.1 - 1 {
                adjacent.push((col - 1, row + 1));
            }
        }
        if col < self.dim.0 - 1 {
            if row > 0 {
                adjacent.push((col + 1, row - 1));
            }
            adjacent.push((col + 1, row));
            if row < self.dim.1 - 1 {
                adjacent.push((col + 1, row + 1));
            }
        }
        if row > 0 {
            adjacent.push((col, row - 1));
        }
        if row < self.dim.1 - 1 {
            adjacent.push((col, row + 1));
        }

        adjacent
    }

    pub fn generate_mines(&mut self, rng: &mut ThreadRng) {
        let (mut rand_col, mut rand_row): Coords<usize>;

        for _ in 0..self.mines {
            loop {
                rand_col = rng.gen_range(0..self.dim.0);
                rand_row = rng.gen_range(0..self.dim.1);
                if !self.get((rand_col, rand_row)).unwrap().is_mine {
                    break;
                }
            }

            self.map[rand_row][rand_col].set_mine();
        }
    }

    pub fn generate_tiles(&mut self) {
        for (i, row) in self.map.clone().iter().enumerate() {
            for (j, tile) in row.iter().enumerate() {
                if tile.is_mine {
                    continue;
                }
                let adjacent = self.get_adjacent_tiles((j, i));
                let mut mines = 0;
                for adj in adjacent {
                    if self.get(adj).unwrap().is_mine {
                        mines += 1;
                    }
                }
                self.map[i][j].value = Some(mines);
            }
        }
    }

    pub fn mine(&mut self, pos: Coords<usize>, prev: &mut [Coords<usize>]) {
        let tile = &mut self.map[pos.1][pos.0];
        if tile.is_mined || tile.is_flagged {
            return;
        }
        if tile.mine() {
            self.lost = true;
        } else if tile.value.unwrap() == 0 {
            let mut adjacent = self.get_adjacent_tiles(pos);
            adjacent.retain(|e| !prev.contains(e));
            for adj in adjacent {
                let mut copy = prev.to_owned();
                copy.push(adj);
                self.mine(adj, &mut copy);
            }
        }
    }

    pub fn flag(&mut self, pos: Coords<usize>) {
        let tile = &mut self.map[pos.1][pos.0];
        if tile.is_mined {
            return;
        }
        if self.flags > 0 || tile.is_flagged {
            if let Some(flagged) = tile.flag() {
                if flagged {
                    self.flags += 1;
                } else {
                    self.flags -= 1;
                }
                self.flags_text.set_text(&format!("Flags: {}", self.flags));
            }
        }
    }

    pub fn check_state(&self) -> GameState {
        if self.lost {
            return GameState::Lose;
        }

        for row in self.map.iter() {
            for tile in row {
                if !tile.is_mined && !tile.is_mine {
                    return GameState::Playing;
                }
            }
        }

        GameState::Win
    }

    pub fn render(
        &mut self,
        canvas: &mut WindowCanvas,
        tex: &Texture,
        font: &Font,
        tex_creator: &'a TextureCreator<WindowContext>,
    ) -> Result<(), String> {
        for (i, row) in self.map.iter().enumerate() {
            for (j, tile) in row.iter().enumerate() {
                tile.render(
                    canvas,
                    tex,
                    (i * TILE_SIZE as usize) as _,
                    (j * TILE_SIZE as usize) as _,
                )?;
            }
        }

        self.flags_text.render(canvas, font, tex_creator)
    }

    pub fn inside(&self, x: i32, y: i32) -> Option<Coords<usize>> {
        if (x as usize) > self.dim.1 * TILE_SIZE as usize
            || (y as usize) > self.dim.0 * TILE_SIZE as usize
        {
            None
        } else {
            Some(((y / TILE_SIZE) as usize, (x / TILE_SIZE) as usize))
        }
    }
}
