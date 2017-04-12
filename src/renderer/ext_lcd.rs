use core::ptr;
use stm32f7::lcd::Lcd;

pub struct LcdExt<'a> {
    pub lcd: &'a mut Lcd,
}

impl<'a> LcdExt<'a> {
    pub fn new(lcd: &'a mut Lcd) -> LcdExt<'a> {
        LcdExt { lcd: lcd }
    }

    pub fn print_point_color_layer_one_at(&mut self, x: u16, y: u16, color: u16) {
        assert!(x < 480);
        assert!(y < 272);

        // layer 1
        let addr: u32 = 0xC000_0000;
        let pixel = u32::from(y) * 480 + u32::from(x);
        let pixel_color = (addr + pixel * 2) as *mut u16;

        unsafe { ptr::write_volatile(pixel_color, color) };
    }

    pub fn print_point_color_layer_two_at(&mut self, x: u16, y: u16, color: u16) {
        assert!(x < 480);
        assert!(y < 272);

        // layer 2
        let addr: u32 = 0xC000_0000 + (480 * 272 * 2);
        let pixel = u32::from(y) * 480 + u32::from(x);
        let pixel_color = (addr + pixel * 2) as *mut u16;

        unsafe { ptr::write_volatile(pixel_color, color) };
    }
}