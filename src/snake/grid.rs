use collections::vec::Vec;
use super::Color;
use super::Tile;

pub struct Grid {
    width: u16,
    height: u16,
    grid: Vec<Tile>,
}

impl Grid {
    pub fn new(width: u16, height: u16) -> Grid {
        let mut grid = Grid {
            width: width,
            height: height,
            grid: Vec::with_capacity((width * height) as usize),
        };
        grid.init_grid();
        grid
    }

    pub fn init_grid(&mut self) {
        self.grid.clear();

        for y in 0..self.height {
            for x in 0..self.width {
                self.grid.push(Tile::Empty);
            }
        }
    }

    pub fn get_height(&self) -> u16 {
        self.height
    }

    pub fn get_width(&self) -> u16 {
        self.width
    }

    pub fn set_tile(&mut self, x: u16, y: u16, tile: Tile) {
        self.grid[(x + y * self.width) as usize] = tile;
    }

    pub fn get_tile(&mut self, x: u16, y: u16) -> Tile {
        assert!(x < self.width);
        assert!(y < self.height);

        self.grid[(x + y * self.width) as usize]
    }

    pub fn get_tile_mut(&mut self, x: u16, y: u16) -> &mut Tile {
        assert!(x < self.width);
        assert!(y < self.height);

        &mut self.grid[(x + y * self.width) as usize]
    }
}