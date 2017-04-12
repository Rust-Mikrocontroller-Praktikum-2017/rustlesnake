use collections::vec::Vec;

use super::Tile;
use super::Position;
use super::Direction;
use super::Snake;
use super::Grid;
use randomizer::Randomizer;
use renderer::*;

const POWER_UP_ENABLED: bool = false;
const POWER_UP_SPAWN_PROB: u16 = 10;

pub struct Game<'a, 'b> {
    renderer: Renderer<'a>,
    randomizer: &'b mut Randomizer,
    width: u16,
    height: u16,
    snake: Snake,
    grid: Grid,
    food: Position,
    food_spawned: bool,
    game_over: bool,
    score: u16,
    seven_segments: Vec<SevenSegment>,
    power_up: Position,
    power_up_spawned: bool,
}

impl<'a, 'b> Game<'a, 'b> {
    pub fn new(renderer: Renderer<'a>,
               randomizer: &'b mut Randomizer,
               width: u16,
               height: u16)
               -> Game<'a, 'b> {
        Game {
            renderer: renderer,
            randomizer: randomizer,
            width: width,
            height: height,
            snake: Snake::new(),
            grid: Grid::new(width, height),
            food: Position::from(0, 0),
            food_spawned: false,
            game_over: false,
            score: 0,
            seven_segments: vec![SevenSegment::new(460, 5, 0),
                                 SevenSegment::new(440, 5, 0),
                                 SevenSegment::new(420, 5, 0)],
            power_up: Position::from(0, 0),
            power_up_spawned: false,
        }
    }

    pub fn restart(&mut self) {
        self.snake = Snake::new();
        self.grid.init_grid();
        self.food = Position::from(0, 0);
        self.food_spawned = false;
        self.game_over = false;
        self.score = 0;
        self.power_up = Position::from(0, 0);
        self.power_up_spawned = false;

        for ss in &mut self.seven_segments {
            ss.set_digit(0);
        }

        self.init_game();
    }

    pub fn init_game(&mut self) {
        // self.renderer.render_grid(&mut self.grid);
        self.renderer.clear_screen();
        self.renderer.render_game_screen();
        self.add_item(Tile::Food);
        self.snake
            .render_completely(&mut self.renderer, &mut self.grid);

        self.renderer.enable_control_overlay();

        for mut ss in &mut self.seven_segments {
            self.renderer.render_score(&mut ss);
        }
    }

    fn add_item(&mut self, item_element: Tile) {
        // self.renderer.render_game_screen();

        let h = self.grid.get_height();
        let w = self.grid.get_width();

        let random = self.randomizer.randomize();
        let r_y = (random % h) as u16;
        let r_x = (random % w) as u16;

        for y in 0..h {
            for x in 0..w {
                let new_x = (r_x + x) % w;
                let new_y = (r_y + y) % h;

                if self.grid.get_tile(new_x, new_y) == Tile::Empty {
                    match item_element {
                        Tile::Food => {
                            self.food = Position::from(new_x, new_y);
                            self.food_spawned = true;
                        }
                        Tile::PowerUp => {
                            self.power_up = Position::from(new_x, new_y);
                            self.power_up_spawned = true;
                        }
                        _ => return,
                    }

                    self.grid.set_tile(new_x, new_y, item_element);
                    self.renderer.render_tile(new_x, new_y, item_element);
                    return;
                }
            }
        }
    }

    fn check_game_end_condition(&self) -> bool {
        self.check_borders() || self.check_snake_body()
    }

    fn check_borders(&self) -> bool {
        let (x, y) = self.snake.get_head_position();

        match self.snake.get_direction() {
            Direction::Up => y == 0,
            Direction::Down => y == self.grid.get_height() - 1,
            Direction::Left => x == 0,
            Direction::Right => x == self.grid.get_width() - 1,
            _ => false,
        }
    }

    fn check_snake_body(&self) -> bool {
        let (x, y) = self.snake.get_head_position();

        let (new_x, new_y) = match self.snake.get_direction() {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
            _ => (x, y),
        };

        self.snake.does_contain(new_x, new_y)
    }

    fn check_food_reached(&mut self) {
        let (x, y) = self.snake.get_head_position();

        if self.food_spawned && x == self.food.x && y == self.food.y {
            self.snake.grow();
            self.food_spawned = false;
            self.score += 1;
            self.update_seven_segments();

            // self.renderer.render_food_screen();
        }
    }

    fn update_seven_segments(&mut self) {
        let mut rem = 1;

        for mut ss in &mut self.seven_segments {
            ss.set_digit((self.score / rem) % 10);
            self.renderer.render_score(&mut ss);

            rem *= 10;
        }
    }

    fn check_power_up_reached(&mut self) {
        let (x, y) = self.snake.get_head_position();

        if self.power_up_spawned && x == self.power_up.x && y == self.power_up.y {
            self.power_up_spawned = false;
        }
    }

    pub fn get_score(&self) -> u16 {
        self.score
    }

    pub fn set_direction(&mut self, direction: Direction) -> bool {
        self.snake.set_direction(direction)
    }

    fn should_spawn_power_up(&mut self) -> bool {
        self.randomizer.randomize() % POWER_UP_SPAWN_PROB == 0
    }

    pub fn disable_control_overlay(&mut self) {
        self.renderer.disable_control_overlay();
    }

    pub fn step(&mut self) {

        if self.game_over {
            return;
        }

        // spawn new food
        if !self.food_spawned {
            self.add_item(Tile::Food);

            // sometimes spawn new power up if enabled
            if POWER_UP_ENABLED {
                if !self.power_up_spawned && self.should_spawn_power_up() {
                    self.add_item(Tile::PowerUp);
                }
            }
        }

        // move snake if game end conditions are false
        if !self.check_game_end_condition() {
            self.snake.make_move();
            self.check_food_reached();

            if POWER_UP_ENABLED {
                self.check_power_up_reached()
            }

            self.snake.render(&mut self.renderer, &mut self.grid);
        } else {
            self.renderer.render_game_over_screen();
            self.game_over = true;
        }
    }
}