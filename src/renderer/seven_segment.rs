use collections::vec::Vec;

use super::Segment;
use super::Rectangle;

pub struct SevenSegment {
    x: u16,
    y: u16,
    digit: u16,
}

impl SevenSegment {
    pub fn new(x: u16, y: u16, digit: u16) -> SevenSegment {
        assert!(digit <= 9);

        SevenSegment {
            x: x,
            y: y,
            digit: digit,
        }
    }

    pub fn set_digit(&mut self, digit: u16) {
        assert!(digit <= 9);
        self.digit = digit;
    }

    pub fn get_x(&mut self) -> u16 {
        self.x
    }

    pub fn get_y(&mut self) -> u16 {
        self.y
    }

    pub fn increment(&mut self) {
        self.digit += 1;
    }

    pub fn decrement(&mut self) {
        if self.digit > 0 {
            self.digit -= 1;
        } else {
            // panic
        }
    }

    pub fn get_segments(&mut self) -> Vec<Segment> {
        Self::segments_for_digit(self.digit)
    }

    pub fn segments_for_digit(digit: u16) -> Vec<Segment> {
        match digit {
            0 => vec![Segment::A, Segment::B, Segment::C, Segment::D, Segment::E, Segment::F],
            1 => vec![Segment::B, Segment::C],
            2 => vec![Segment::A, Segment::B, Segment::G, Segment::E, Segment::D],
            3 => vec![Segment::A, Segment::B, Segment::G, Segment::C, Segment::D],
            4 => vec![Segment::F, Segment::B, Segment::G, Segment::C],
            5 => vec![Segment::A, Segment::F, Segment::G, Segment::C, Segment::D],
            6 => vec![Segment::A, Segment::C, Segment::D, Segment::E, Segment::F, Segment::G],
            7 => vec![Segment::A, Segment::B, Segment::C],
            8 => {
                vec![Segment::A, Segment::B, Segment::C, Segment::D, Segment::E, Segment::F,
                     Segment::G]
            }
            9 => vec![Segment::A, Segment::B, Segment::C, Segment::D, Segment::F, Segment::G],
            _ => vec![],
        }
    }
}