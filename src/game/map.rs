use crate::stopwatch::Stopwatch;
use rand::{rngs::ThreadRng, Rng};
use sdl2::{
    render::{Texture, TextureCreator, WindowCanvas},
    ttf::Font,
    video::WindowContext,
};

use crate::ui::text::Text;

use super::{
    tile::{Tile, TILE_SIZE},
    Coords, Stage,
};

pub enum Size {
    Small,
    Normal,
    Large,
}

pub struct Map<'a> {
    dim: Coords<i32>,
    map: Vec<Vec<Tile>>,
    lost: bool,
    pub first_move: bool,
    mines: u8,
    flags: u8,
    flags_text: Text<'a>,
    time_text: Text<'a>,
    pub stopwatch: Stopwatch,
}

impl<'a> Map<'a> {
    pub fn new(
        size: &Size,
        tex_creator: &'a TextureCreator<WindowContext>,
        font: &'a Font,
    ) -> Map<'a> {
        let dim = match size {
            Size::Small => (9, 9),
            Size::Normal => (16, 16),
            Size::Large => (18, 30),
        };
        let mines = match size {
            Size::Small => 10,
            Size::Normal => 40,
            Size::Large => 99,
        };
        Map {
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
            first_move: true,
            mines,
            flags: mines,
            flags_text: Text::new(0, 0, tex_creator, &format!("Flags: {mines}"), font),
            time_text: Text::new(
                0,
                match size {
                    Size::Small => 9,
                    Size::Normal => 16,
                    Size::Large => 18,
                } * TILE_SIZE
                    - 30,
                tex_creator,
                "Time: 0",
                font,
            ),
            stopwatch: Stopwatch::default(),
        }
    }

    fn get(&self, pos: Coords<i32>) -> Option<&Tile> {
        if pos.0 > self.dim.0 - 1 || pos.1 > self.dim.1 - 1 {
            None
        } else {
            Some(&self.map[usize::try_from(pos.1).unwrap()][usize::try_from(pos.0).unwrap()])
        }
    }

    fn get_adjacent_tiles(&self, pos: Coords<i32>) -> Vec<Coords<i32>> {
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

    pub fn generate_mines(&mut self, rng: &mut ThreadRng, x: i32, y: i32) {
        let (mut rand_col, mut rand_row): Coords<i32>;

        for _ in 0..self.mines {
            loop {
                rand_col = rng.gen_range(0..self.dim.0);
                rand_row = rng.gen_range(0..self.dim.1);
                if !(self.get((rand_col, rand_row)).unwrap().is_mine
                    || (x - rand_col).abs() < 2
                    || (y - rand_row).abs() < 2)
                {
                    break;
                }
            }

            self.map[usize::try_from(rand_row).unwrap()][usize::try_from(rand_col).unwrap()]
                .set_mine();
        }
    }

    pub fn generate_tiles(&mut self) {
        for (i, row) in self.map.clone().iter().enumerate() {
            for (j, tile) in row.iter().enumerate() {
                if tile.is_mine {
                    continue;
                }
                let adjacent =
                    self.get_adjacent_tiles((i32::try_from(j).unwrap(), i32::try_from(i).unwrap()));
                let mut mines = 0;
                for adj in adjacent {
                    if self.get(adj).unwrap().is_mine {
                        mines += 1;
                    }
                }
                self.map[i][j].set_value(mines);
            }
        }
    }

    pub fn mine(&mut self, pos: Coords<i32>, prev: &mut [Coords<i32>]) {
        let tile = &mut self.map[usize::try_from(pos.1).unwrap()][usize::try_from(pos.0).unwrap()];
        if tile.is_flagged {
        } else if tile.is_mined {
            let adjacent = self.get_adjacent_tiles(pos);
            let (flags, non_flags): (Vec<Coords<i32>>, Vec<Coords<i32>>) = adjacent
                .iter()
                .filter(|&&e| !self.get(e).unwrap().is_mined)
                .partition(|&&e| self.get(e).unwrap().is_flagged);
            if u8::try_from(flags.len()).unwrap() == self.get(pos).unwrap().value.unwrap() {
                for t in non_flags {
                    self.mine(t, &mut []);
                }
            }
        } else if tile.mine() {
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

    pub fn flag(&mut self, pos: Coords<i32>) {
        let tile = &mut self.map[usize::try_from(pos.1).unwrap()][usize::try_from(pos.0).unwrap()];
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

    pub fn check_state(&self) -> Stage {
        if self.lost {
            return Stage::Lose;
        }

        for row in &self.map {
            for tile in row {
                if !tile.is_mined && !tile.is_mine {
                    return Stage::Playing;
                }
            }
        }

        Stage::Win
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
                    i32::try_from(i).unwrap() * TILE_SIZE,
                    i32::try_from(j).unwrap() * TILE_SIZE,
                )?;
            }
        }

        self.flags_text.render(canvas, font, tex_creator)?;

        self.time_text
            .set_text(&format!("Time: {}", self.stopwatch.elapsed().as_secs()));

        self.time_text.render(canvas, font, tex_creator)?;

        Ok(())
    }

    pub fn inside(&self, x: i32, y: i32) -> Option<Coords<i32>> {
        if x > self.dim.1 * TILE_SIZE || y > self.dim.0 * TILE_SIZE {
            None
        } else {
            Some((y / TILE_SIZE, x / TILE_SIZE))
        }
    }
}
