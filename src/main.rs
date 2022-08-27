use rand::{rngs::ThreadRng, Rng};
use std::fmt;

enum MapSize {
    Small,
    Normal,
    Large,
}

#[derive(Clone)]
struct Tile {
    value: Option<u8>,
    is_mine: bool,
    is_flagged: bool,
}
impl Tile {
    fn new(value: Option<u8>) -> Self {
        Self {
            value,
            is_mine: value.is_none(),
            is_flagged: false,
        }
    }
}
impl fmt::Debug for Tile {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_fmt(format_args!(
            "{}",
            if self.is_mine {
                "M".to_string()
            } else if self.is_flagged {
                "F".to_string()
            } else {
                self.value.unwrap().to_string()
            }
        ))
    }
}

type Coords = (usize, usize);

struct Map {
    size: MapSize,
    dim: Coords,
    map: Vec<Vec<Tile>>,
}
impl Map {
    fn new(size: MapSize) -> Self {
        let (cols, rows): (usize, usize) = match size {
            MapSize::Small => (9, 9),
            MapSize::Normal => (16, 16),
            MapSize::Large => (30, 18),
        };
        Self {
            size,
            dim: (cols, rows),
            map: vec![vec![Tile::new(Some(0)); cols]; rows],
        }
    }

    fn get_adjacent_tiles(&self, pos: Coords) -> Vec<Coords> {
        let mut adjacent = vec![];
        let (col, row) = pos;
        if col > 0 {
            if row > 0 {
                adjacent.push((col - 1, row - 1));
            }
            adjacent.push((col - 1, row));
            if row < self.dim.1 {
                adjacent.push((col - 1, row + 1));
            }
        }
        if col < self.dim.0 {
            if row > 0 {
                adjacent.push((col + 1, row - 1));
            }
            adjacent.push((col + 1, row));
            if row < self.dim.1 {
                adjacent.push((col + 1, row + 1));
            }
        }
        if row > 0 {
            adjacent.push((col, row - 1));
        }
        if row < self.dim.1 {
            adjacent.push((col, row + 1));
        }

        adjacent
    }

    fn generate_mines(&mut self, rng: &mut ThreadRng) {
        let num_mines = match self.size {
            MapSize::Small => 10,
            MapSize::Normal => 40,
            MapSize::Large => 99,
        };

        let (mut rand_col, mut rand_row): Coords;

        for _ in 0..num_mines {
            loop {
                rand_col = rng.gen_range(0..self.dim.0);
                rand_row = rng.gen_range(0..self.dim.1);
                if !self.map[rand_row][rand_col].is_mine {
                    break;
                }
            }

            self.map[rand_row][rand_col] = Tile::new(None);
        }
    }

    fn generate_tiles(&mut self) {
        for (i, row) in self.map.iter().enumerate() {
            for (j, tile) in row.iter().enumerate() {}
        }
    }

    fn mine(&mut self, pos: Coords, prev: &mut Vec<Coords>) -> bool {
        let (col, row) = pos;
        let tile = &self.map[row][col];
        if tile.is_mine {
            return true;
        } else if tile.value.unwrap() == 0 {
            let mut adjacent = self.get_adjacent_tiles(pos);
            adjacent.retain(|e| !prev.contains(e));
            for adj in adjacent {
                let mut copy = prev.clone();
                copy.push(adj);
                self.mine(adj, &mut copy);
            }
        }

        false
    }
}
impl fmt::Debug for Map {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        for row in self.map.clone() {
            for tile in row {
                formatter.write_fmt(format_args!("{:?} ", tile))?;
            }
            formatter.write_str("\n")?;
        }

        Ok(())
    }
}

fn main() {
    let mut map = Map::new(MapSize::Normal);
    map.generate_mines(&mut rand::thread_rng());
    println!("{:?}", map);
}
