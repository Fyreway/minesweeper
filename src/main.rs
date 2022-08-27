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
            map: vec![vec![Tile::new(Some(0)); cols]; rows],
        }
    }

    fn generate(&mut self) {
        todo!();
    }

    fn mine(&mut self, col: usize, row: usize, prev: &mut Vec<Coords>) -> bool {
        let tile = &self.map[row][col];
        if tile.is_mine {
            return true;
        } else if tile.value.unwrap() == 0 {
            let mut adjacent = vec![
                (col - 1, row - 1),
                (col - 1, row),
                (col - 1, row + 1),
                (col, row - 1),
                (col, row + 1),
                (col + 1, row - 1),
                (col + 1, row),
                (col + 1, row + 1),
            ];
            adjacent.retain(|e| !prev.contains(e));
            for adj in adjacent {
                let mut copy = prev.clone();
                copy.push(adj);
                let (adj_col, adj_row) = adj;
                self.mine(adj_col, adj_row, &mut copy);
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
    let map = Map::new(MapSize::Normal);
    println!("Hello, world!");
}
