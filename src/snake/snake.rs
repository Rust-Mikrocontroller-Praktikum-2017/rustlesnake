use collections::vec_deque::VecDeque;

use renderer::Renderer;
use super::*;

const SNAKE_START_X: u16 = 10;
const SNAKE_START_Y: u16 = 5;
const SNAKE_START_LENGTH: u16 = 4;

pub struct Snake {
    body: VecDeque<Position>,
    removed_tail: Option<Position>,
    direction: Direction,
    next_direction: Direction,
}

impl Snake {
    pub fn new() -> Snake {
        let mut snake = Snake {
            body: VecDeque::new(),
            removed_tail: None,
            direction: Direction::Right,
            next_direction: Direction::Right,
        };

        snake.init_snake();
        snake
    }

    fn init_snake(&mut self) {
        for i in 0..SNAKE_START_LENGTH {
            self.body
                .push_back(Position::from(SNAKE_START_X - i, SNAKE_START_Y));
        }
    }

    pub fn render_completely(&mut self, renderer: &mut Renderer, grid: &mut Grid) {
        for b in &self.body {
            grid.set_tile(b.x, b.y, Tile::Snake);
            renderer.render_tile(b.x, b.y, Tile::Snake);
        }
    }

    pub fn render(&mut self, renderer: &mut Renderer, grid: &mut Grid) {
        // render new head
        {
            let head = self.body.front().unwrap();
            grid.set_tile(head.x, head.y, Tile::Snake);
            renderer.render_tile(head.x, head.y, Tile::Snake);
        }

        // render new empty tile behind tail
        match self.removed_tail {
            Some(ref t) => {
                grid.set_tile(t.x, t.y, Tile::Empty);
                renderer.render_tile(t.x, t.y, Tile::Empty);
            }
            None => {}
        }
    }

    fn add_new_head(&mut self, head: Position) {
        self.body.push_front(head);
    }

    pub fn grow(&mut self) {
        self.body.push_back(self.removed_tail.clone().unwrap());
        self.removed_tail = None;
    }

    pub fn get_direction(&self) -> Direction {
        self.next_direction
    }

    pub fn set_direction(&mut self, direction: Direction) -> bool {
        if direction == Direction::None || self.direction == direction ||
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
            Direction::Up => self.add_new_head(Position::from(x, y - 1)),
            Direction::Down => self.add_new_head(Position::from(x, y + 1)),
            Direction::Left => self.add_new_head(Position::from(x - 1, y)),
            Direction::Right => self.add_new_head(Position::from(x + 1, y)),
            Direction::None => return,
        }

        self.removed_tail = self.body.pop_back();
    }
}