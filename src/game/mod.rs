pub mod map;
pub mod tile;

pub type Coords<T> = (T, T);

#[derive(PartialEq, Eq)]
pub enum Stage {
    Lose,
    Playing,
    Win,
}
