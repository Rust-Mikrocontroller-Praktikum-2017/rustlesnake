use board;

pub struct Rng {
    rng: &'static board::rng::Rng
}

impl Rng {
    pub fn init(rng: &'static mut board::rng::Rng, rcc: &'static mut board::rcc::Rcc) -> Rng {
        rcc.ahb2enr.update(|r| {
            // Random number generator clock enable
            r.set_rngen(true);
        });
        rng.cr.update(|r| {
            // Interrupt disable
            r.set_ie(false);
            // Random number generator enable
            r.set_rngen(true);
        });
        
        self::Rng {
            rng: rng
        }
    }

    pub fn next_u32(&self) -> u32 {
        // Wait for data
        while !self.rng.sr.read().drdy() {}

        self.rng.dr.read().rndata()
    }
}

pub trait Randomizer {
    fn randomize(&mut self) -> u16;
}

impl Randomizer for Rng {
    fn randomize(&mut self) -> u16 {
        self.next_u32() as u16
    }
}