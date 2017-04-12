pub enum Color {
    Background,
    GameOver,
    SevenSegment,
    ControlOverlay,
    Snake,
    Food,
    PowerUp,
}

impl Color {
    pub fn value(&self) -> u16 {
        match *self {
            Color::Background => 0x0000,
            Color::GameOver => 0xBC00,
            Color::SevenSegment => 0xFFFF,
            Color::ControlOverlay => 0x9CE7,
            Color::Snake => 0x83E0,
            Color::Food => 0xDC1F,
            Color::PowerUp => 0xBBBB,
        }
    }
}