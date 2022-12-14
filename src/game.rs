use rand::{rngs::ThreadRng, Rng};
use sdl2::{
    mouse::MouseState,
    rect::Rect,
    render::{Texture, WindowCanvas},
};
// use std::fmt;

pub enum MapSize {
    Small,
    Normal,
    Large,
}

#[derive(Clone)]
struct Tile {
    value: Option<u8>,
    is_mine: bool,
    is_flagged: bool,
    is_mined: bool,
    rect: Rect,
}
impl Tile {
    fn new(value: Option<u8>) -> Tile {
        Tile {
            value,
            is_mine: value.is_none(),
            is_flagged: false,
            is_mined: false,
            rect: Rect::new(0, 16, 16, 16),
        }
    }

    fn set_mine(&mut self) {
        self.value = None;
        self.is_mine = true;
    }

    fn set_value(&mut self, value: u8) {
        self.is_mine = false;
        self.value = Some(value);
    }

    fn mine(&mut self) -> bool {
        self.is_mined = true;
        if self.is_mine {
            self.rect.x = 32;
            true
        } else {
            self.rect.x = 16 * self.value.unwrap() as i32;
            self.rect.y = 0;
            false
        }
    }

    fn flag(&mut self) -> bool {
        if !self.is_mined && {
            self.is_flagged = !self.is_flagged;
            self.is_flagged
        } {
            self.rect.x = 16;
            false
        } else {
            true
        }
    }

    fn render(
        &self,
        canvas: &mut WindowCanvas,
        tex: &Texture,
        x: i32,
        y: i32,
    ) -> Result<(), String> {
        canvas.copy(tex, self.rect, Rect::new(x, y, 16, 16))
    }
}
// impl fmt::Debug for Tile {
//     fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//         formatter.write_fmt(format_args!(
//             "{}",
//             if self.is_mine {
//                 "M".to_string()
//             } else if self.is_flagged {
//                 "F".to_string()
//             } else {
//                 self.value.unwrap().to_string()
//             }
//         ))
//     }
// }

pub type Coords<T> = (T, T);

pub enum GameState {
    Lose,
    Playing,
    Win,
}

pub struct Map {
    size: MapSize,
    dim: Coords<usize>,
    map: Vec<Vec<Tile>>,
    lost: bool,
    mines: u8,
    flags: u8,
}
impl Map {
    pub fn new(size: MapSize) -> Map {
        let dim = match size {
            MapSize::Small => (9, 9),
            MapSize::Normal => (16, 16),
            MapSize::Large => (30, 18),
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
        }
    }

    fn get(&self, pos: Coords<usize>) -> Option<&Tile> {
        if pos.0 > self.dim.0 - 1 || pos.1 > self.dim.1 - 1 {
            None
        } else {
            Some(&self.map[pos.1 as usize][pos.0 as usize])
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

    pub fn mine(&mut self, pos: Coords<usize>, prev: &mut Vec<Coords<usize>>) {
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
                let mut copy = prev.clone();
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
            if tile.flag() {
                self.flags += 1;
            } else {
                self.flags -= 1;
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

    pub fn render(&self, canvas: &mut WindowCanvas, tex: &Texture) -> Result<(), String> {
        for (i, row) in self.map.iter().enumerate() {
            for (j, tile) in row.iter().enumerate() {
                tile.render(canvas, tex, (i * 16) as _, (j * 16) as _)?;
            }
        }

        Ok(())
    }

    pub fn inside(&self, m: &MouseState) -> Option<Coords<usize>> {
        if (m.x() as usize) > self.dim.0 * 16 || (m.y() as usize) > self.dim.1 * 16 {
            None
        } else {
            Some((m.y() as usize / 16, m.x() as usize / 16))
        }
    }
}
// impl fmt::Debug for Map {
//     fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//         for row in self.map.iter() {
//             for tile in row {
//                 formatter.write_fmt(format_args!("{:?} ", tile))?;
//             }
//             formatter.write_str("\n")?;
//         }

//         Ok(())
//     }
// }
