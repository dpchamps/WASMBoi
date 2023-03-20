#[derive(Default)]
pub struct Clock {
    cycles: usize,
}

impl Clock {
    pub fn reset(&mut self) {
        self.cycles = 0;
    }

    pub fn add_cycles(&mut self, cycles: u8) {
        self.cycles += (cycles as usize);
    }
}
