pub mod map;
pub mod tile;

pub type Coords<T> = (T, T);

pub enum Stage {
    Lose,
    Playing,
    Win,
}
