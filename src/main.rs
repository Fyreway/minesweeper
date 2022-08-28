#![allow(dead_code)]
use rand::{rngs::ThreadRng, Rng};
use sdl2::{event::Event, image::InitFlag};
use std::{fmt, path::PathBuf, time::Duration};

mod button;
use button::*;

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
    is_mined: bool,
}
impl Tile {
    fn new(value: Option<u8>) -> Self {
        Self {
            value,
            is_mine: value.is_none(),
            is_flagged: false,
            is_mined: false,
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

enum GameState {
    Lose,
    Playing,
    Win,
}

struct Map {
    size: MapSize,
    dim: Coords,
    map: Vec<Vec<Tile>>,
    lost: bool,
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
            lost: false,
        }
    }

    fn get(&self, pos: Coords) -> Option<&Tile> {
        if pos.0 > self.dim.0 - 1 || pos.1 > self.dim.1 - 1 {
            None
        } else {
            Some(&self.map[pos.1][pos.0])
        }
    }

    fn get_mut(&mut self, pos: Coords) -> Option<&mut Tile> {
        if pos.0 > self.dim.0 - 1 || pos.1 > self.dim.1 - 1 {
            None
        } else {
            Some(&mut self.map[pos.1][pos.0])
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

    fn mine(&mut self, pos: Coords, prev: &mut Vec<Coords>) {
        let mut tile = self.get_mut(pos).unwrap();
        tile.is_mined = true;
        if tile.is_mine {
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

    fn check_state(&self) -> GameState {
        if self.lost {
            return GameState::Lose;
        }

        for row in self.map.clone() {
            for tile in row {
                if !tile.is_mined && !tile.is_mine {
                    return GameState::Playing;
                }
            }
        }

        GameState::Win
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

fn generate(size: MapSize) -> Map {
    let mut map = Map::new(size);
    map.generate_mines(&mut rand::thread_rng());
    map.generate_tiles();
    map
}

fn main() {
    let sdl = sdl2::init().expect("Could not start SDL");
    let video = sdl.video().expect("Could not start video subsystem");
    let _img = sdl2::image::init(InitFlag::PNG).expect("Could not start SDL_image");
    let window = video
        .window("Minesweeper", 800, 600)
        .position_centered()
        .build()
        .expect("Could not create window");
    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .expect("Could not create canvas");
    let mut tex_creator = canvas.texture_creator();
    let mut events = sdl.event_pump().expect("Could not get event pump");
    let btn = Button::new(
        100,
        100,
        128,
        5,
        &mut tex_creator,
        PathBuf::from("res/button.png"),
    );

    'gameloop: loop {
        for e in events.poll_iter() {
            match e {
                Event::Quit { .. } => break 'gameloop,
                _ => (),
            }
        }
        // match map.check_state() {
        //     GameState::Win => {
        //         // TODO: Display win state
        //         break 'gameloop;
        //     }
        //     GameState::Lose => {
        //         // TODO: Display lose state
        //         break 'gameloop;
        //     }
        //     GameState::Playing => {}
        // }

        canvas.clear();

        btn.render(&mut canvas).expect("Could not render button");

        canvas.present();
        std::thread::sleep(Duration::from_nanos(1_000_000_000u64 / 60));
    }
}
