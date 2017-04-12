use collections::vec::Vec;
use stm32f7::lcd;

use snake::*;
use renderer::*;

const CENTER_X: u16 = 480 / 2;
const CENTER_Y: u16 = 272 / 2;
const T_HEIGHT: u16 = 20;
const T_WIDTH: u16 = 40;


pub struct Renderer<'a> {
    lcd_ext: LcdExt<'a>,
}

impl<'a> Renderer<'a> {
    pub fn new(lcd_ext: LcdExt<'a>) -> Renderer<'a> {
        Renderer { lcd_ext: lcd_ext }
    }

    pub fn render_grid(&mut self, grid: &mut Grid) {
        for y in 0..grid.get_height() {
            for x in 0..grid.get_width() {
                self.render_tile(&grid.get_tile(x, y));
            }
        }
    }

    pub fn render_tile(&mut self, tile: &Tile) {
        let color = tile.get_tile_element().get_design();

        for y in tile.get_y()..(tile.get_y() + tile.get_length()) {
            for x in tile.get_x()..(tile.get_x() + tile.get_length()) {
                self.lcd_ext.print_point_color_layer_one_at(x, y, color);
            }
        }
    }

    pub fn clear_screen(&mut self) {
        self.lcd_ext.lcd.clear_screen();
    }

    pub fn render_game_screen(&mut self) {
        self.lcd_ext.lcd
            .set_background_color(lcd::Color::from_argb1555(Color::Background.value()));
    }

    pub fn render_food_screen(&mut self) {
        self.lcd_ext.lcd
            .set_background_color(lcd::Color::from_argb1555(Color::Food.value()));
    }

    pub fn render_game_over_screen(&mut self) {
        self.lcd_ext.lcd
            .set_background_color(lcd::Color::from_argb1555(Color::GameOver.value()));
    }

    pub fn enable_control_overlay(&mut self) {
        self.render_controls(Color::ControlOverlay.value());
    }

    pub fn disable_control_overlay(&mut self) {
        self.render_controls(Color::Background.value());
    }

    fn render_controls(&mut self, color: u16) {
        let vertical_offset = 100;
        let horizontal_offset = 200;
        let half_width = T_WIDTH / 2;


        // Up control
        self.render_control(CENTER_Y - T_HEIGHT - vertical_offset,
                            CENTER_Y - vertical_offset,
                            CENTER_X - half_width,
                            CENTER_X + half_width,
                            half_width,
                            ControlAlignment::Vertical,
                            ControlLayout::Up,
                            color);

        // Down control
        self.render_control(CENTER_Y + vertical_offset,
                            CENTER_Y + vertical_offset + T_HEIGHT,
                            CENTER_X - half_width,
                            CENTER_X + half_width,
                            0,
                            ControlAlignment::Vertical,
                            ControlLayout::Down,
                            color);

        // Left control
        self.render_control(CENTER_X - T_HEIGHT - horizontal_offset,
                            CENTER_X - horizontal_offset,
                            CENTER_Y - half_width,
                            CENTER_Y + half_width,
                            half_width,
                            ControlAlignment::Horizontal,
                            ControlLayout::Up,
                            color);

        // Right control
        self.render_control(CENTER_X + horizontal_offset,
                            CENTER_X + horizontal_offset + T_HEIGHT,
                            CENTER_Y - half_width,
                            CENTER_Y + half_width,
                            0,
                            ControlAlignment::Horizontal,
                            ControlLayout::Down,
                            color);
    }

    fn render_control(&mut self,
                      start_x: u16,
                      end_x: u16,
                      start_y: u16,
                      end_y: u16,
                      offset: u16,
                      control_alignment: ControlAlignment,
                      control_layout: ControlLayout,
                      color: u16) {

        let mut offset = offset;

        for y in start_x..end_x {
            for x in start_y + offset..end_y - offset {
                match control_alignment {
                    ControlAlignment::Vertical => {
                        self.lcd_ext.print_point_color_layer_two_at(x, y, color)
                    }
                    ControlAlignment::Horizontal => {
                        self.lcd_ext.print_point_color_layer_two_at(y, x, color)
                    }
                }
            }
            offset = match control_layout {
                ControlLayout::Down => offset + 1,
                ControlLayout::Up => offset - 1,
            }
        }
    }

    pub fn render_score(&mut self, seven_segment: &mut SevenSegment) {
        // reset seven segments with an eight
        for segment in SevenSegment::segments_for_digit(8) {
            self.render_rectangle(seven_segment.get_x(),
                                  seven_segment.get_y(),
                                  segment.get_rectangle(),
                                  Color::Background.value());
        }

        // set new digit
        for segment in seven_segment.get_segments() {
            self.render_rectangle(seven_segment.get_x(),
                                  seven_segment.get_y(),
                                  segment.get_rectangle(),
                                  Color::SevenSegment.value());
        }
    }

    fn render_rectangle(&mut self, start_x: u16, start_y: u16, rectangle: Rectangle, color: u16) {
        for y in 0..rectangle.height {
            for x in 0..rectangle.width {
                self.lcd_ext
                    .print_point_color_layer_two_at(start_x + x + rectangle.upper_left_corner.0,
                                                    start_y + y + rectangle.upper_left_corner.1,
                                                    color);
            }
        }
    }
}