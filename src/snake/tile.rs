use super::Color;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Tile {
    Empty,
    Snake,
    Food,
    PowerUp,
}

impl Tile {
    pub fn get_design(&self) -> u16 {
        match *self {
            Tile::Empty => Color::Background.value(),
            Tile::Snake => Color::Snake.value(),
            Tile::Food => Color::Food.value(),
            Tile::PowerUp => Color::PowerUp.value(),
        }
    }
}