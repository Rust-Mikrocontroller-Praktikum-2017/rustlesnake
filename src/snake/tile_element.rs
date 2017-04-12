use super::Color;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TileElement {
    Empty,
    Snake,
    Food,
    PowerUp,
}

impl TileElement {
    pub fn get_design(&self) -> u16 {
        match *self {
            TileElement::Empty => Color::Background.value(),
            TileElement::Snake => Color::Snake.value(),
            TileElement::Food => Color::Food.value(),
            TileElement::PowerUp => Color::PowerUp.value(),
        }
    }
}