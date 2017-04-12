use super::Rectangle;

pub enum Segment {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl Segment {
    pub fn get_rectangle(&self) -> Rectangle {
        let long = 10;
        let short = 2;
        let space = 1;

        match *self {
            Segment::A => Rectangle::new((short + space, 0), long, short),
            Segment::B => Rectangle::new((short + long + 2 * space, short + space), short, long),
            Segment::C => {
                Rectangle::new((short + long + 2 * space, 2 * short + long + 3 * space),
                               short,
                               long)
            }
            Segment::D => {
                Rectangle::new((short + space, 2 * short + 2 * long + 4 * space),
                               long,
                               short)
            }
            Segment::E => Rectangle::new((0, 2 * short + long + 3 * space), short, long),
            Segment::F => Rectangle::new((0, short + space), short, long),
            Segment::G => Rectangle::new((short + space, short + long + 2 * space), long, short),
        }

    }
}