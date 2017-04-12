use collections::vec_deque::VecDeque;

use renderer::Renderer;
use super::*;

const SNAKE_START_X: u16 = 10;
const SNAKE_START_Y: u16 = 5;
const SNAKE_START_LENGTH: u16 = 4;

pub struct Snake {
    body: VecDeque<TileCoord>,
    removed_tail: Option<TileCoord>,
    direction: MoveDirection,
    next_direction: MoveDirection,
}

impl Snake {
    pub fn new() -> Snake {
        let mut snake = Snake {
            body: VecDeque::new(),
            removed_tail: None,
            direction: MoveDirection::Right,
            next_direction: MoveDirection::Right,
        };

        snake.init_snake();
        snake
    }

    fn init_snake(&mut self) {
        for i in 0..SNAKE_START_LENGTH {
            self.body
                .push_back(TileCoord::from(SNAKE_START_X - i, SNAKE_START_Y));
        }
    }

    pub fn render_completely(&mut self, renderer: &mut Renderer, grid: &mut Grid) {
        for b in &self.body {
            let mut tile = grid.get_tile_mut(b.x, b.y);
            tile.set_tile_element(TileElement::Snake);
            renderer.render_tile(tile);
        }
    }

    pub fn render(&mut self, renderer: &mut Renderer, grid: &mut Grid) {
        // render new head
        {
            let head = self.body.front().unwrap();
            let mut head_tile = grid.get_tile_mut(head.x, head.y);
            head_tile.set_tile_element(TileElement::Snake);
            renderer.render_tile(head_tile);
        }

        // render new empty tile behind tail
        match self.removed_tail {
            Some(ref t) => {
                let mut tile = grid.get_tile_mut(t.x, t.y);
                tile.set_tile_element(TileElement::Empty);
                renderer.render_tile(tile);
            }
            None => {}
        }
    }

    fn add_new_head(&mut self, head: TileCoord) {
        self.body.push_front(head);
    }

    pub fn grow(&mut self) {
        self.body.push_back(self.removed_tail.clone().unwrap());
        self.removed_tail = None;
    }

    pub fn get_direction(&self) -> MoveDirection {
        self.next_direction
    }

    pub fn set_direction(&mut self, direction: MoveDirection) -> bool {
        if direction == MoveDirection::None || self.direction == direction ||
           self.direction == direction.opposite() {
            return false;
        }

        self.next_direction = direction;
        true
    }

    pub fn get_head_position(&self) -> (u16, u16) {
        let head = self.body.front().unwrap();
        (head.x, head.y)
    }

    pub fn does_contain(&self, x: u16, y: u16) -> bool {
        for b in &self.body {
            if b.x == x && b.y == y {
                return true;
            }
        }
        false
    }

    pub fn make_move(&mut self) {
        let x: u16;
        let y: u16;
        {
            let head = self.body.front().unwrap();
            x = head.x;
            y = head.y;
        }

        self.direction = self.next_direction;

        match self.direction {
            MoveDirection::Up => self.add_new_head(TileCoord::from(x, y - 1)),
            MoveDirection::Down => self.add_new_head(TileCoord::from(x, y + 1)),
            MoveDirection::Left => self.add_new_head(TileCoord::from(x - 1, y)),
            MoveDirection::Right => self.add_new_head(TileCoord::from(x + 1, y)),
            MoveDirection::None => return,
        }

        self.removed_tail = self.body.pop_back();
    }
}