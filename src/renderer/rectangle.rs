pub struct Rectangle {
    pub upper_left_corner: (u16, u16),
    pub width: u16,
    pub height: u16,
}

impl Rectangle {
    pub fn new(upper_left_corner: (u16, u16), width: u16, height: u16) -> Rectangle {
        Rectangle {
            upper_left_corner: upper_left_corner,
            width: width,
            height: height,
        }
    }
}