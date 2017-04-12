#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct TileCoord {
    pub x: u16,
    pub y: u16,
}

impl TileCoord {
    pub fn from(x: u16, y: u16) -> TileCoord {
        TileCoord { x: x, y: y }
    }

    pub fn from_tuple(coord: (u16, u16)) -> TileCoord {
        TileCoord {
            x: coord.0,
            y: coord.1,
        }
    }
}