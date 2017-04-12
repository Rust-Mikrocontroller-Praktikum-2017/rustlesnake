use super::TileElement;

#[derive(Debug, Clone, Copy)]
pub struct Tile {
    x: u16,
    y: u16,
    length: u16,
    element: TileElement,
}

impl Tile {
    pub fn new(x: u16, y: u16, length: u16, element: TileElement) -> Tile {
        Tile {
            x: x,
            y: y,
            length: length,
            element: element,
        }
    }

    pub fn get_x(&self) -> u16 {
        self.x
    }

    pub fn get_y(&self) -> u16 {
        self.y
    }

    pub fn get_length(&self) -> u16 {
        self.length
    }

    pub fn get_tile_element(&self) -> TileElement {
        self.element
    }

    pub fn set_tile_element(&mut self, element: TileElement) {
        self.element = element;
    }
}