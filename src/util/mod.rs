mod color;

pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8
}

#[derive(Clone)]
pub enum Cell {
    Empty,
    Food,
    Head,
    Tail
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum GameState {
    Over,
    Playing
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    None
}
