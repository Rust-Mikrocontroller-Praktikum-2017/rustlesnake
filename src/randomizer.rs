pub trait Randomizer {
    fn randomize(&mut self) -> u16;
}

pub struct RNG {
    pub seed: u16,
}

impl Randomizer for RNG {
    fn randomize(&mut self) -> u16 {
        self.seed = self.seed.wrapping_mul(97);
        self.seed
    }
}