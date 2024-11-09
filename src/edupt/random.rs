

pub struct XorShift {
    seed: [u64; 4],
}

impl XorShift {
    pub fn new(initial_seed: u64) -> XorShift {
        let mut seed = [0; 4];
        let mut s = initial_seed;
        for i in 1..5 {
            s = 1812433253u64.wrapping_mul(s ^ (s >> 30)) + i;
            seed[(i - 1) as usize] = s;
        }
        XorShift {seed}
    }
    fn next(&mut self) -> u64 {
        let t = self.seed[0] ^ (self.seed[0] << 11);
        self.seed[0] = self.seed[1];
        self.seed[1] = self.seed[2];
        self.seed[2] = self.seed[3];
        self.seed[3] = (self.seed[3] ^ (self.seed[3] >> 19)) ^ (t ^ (t >> 8));
        self.seed[3]
    }
    pub fn next01(&mut self) -> f64 {
        (self.next() as f64)  / (std::u64::MAX as f64)
    }
}

pub type Random = XorShift;