#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

impl Position {
    pub fn from(x: u16, y: u16) -> Position {
        Position { x: x, y: y }
    }

    pub fn from_tuple(coord: (u16, u16)) -> Position {
        Position {
            x: coord.0,
            y: coord.1,
        }
    }
}