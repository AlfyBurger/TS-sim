//! Separate random streams keyed by (game seed, purpose, side, counter): two versions of a game
//! keep sharing their luck even after their decisions differ.

#[inline]
pub fn splitmix(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9E3779B97F4A7C15);
    let mut z = x;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31)
}

#[derive(Clone, Copy, Debug)]
pub enum Purpose { Shuffle = 1, Coup = 2, Realign = 3, Space = 4, War = 5, EventDie = 6, Random = 7, Trap = 8, Sim = 9, Bot = 10, Setup = 11 }

#[derive(Clone)]
pub struct Streams {
    pub seed: u64,
    pub counters: [[u64; 2]; 12],
}

impl Streams {
    pub fn new(seed: u64) -> Self { Streams { seed, counters: [[0; 2]; 12] } }
    pub fn next(&mut self, p: Purpose, side: usize) -> u64 {
        let pi = p as usize;
        let c = self.counters[pi][side];
        self.counters[pi][side] += 1;
        splitmix(splitmix(self.seed ^ ((pi as u64) << 56) ^ ((side as u64) << 48)) ^ c.wrapping_mul(0xD1B54A32D192ED03))
    }
    pub fn die(&mut self, p: Purpose, side: usize) -> i32 { (self.next(p, side) % 6) as i32 + 1 }
    pub fn below(&mut self, p: Purpose, side: usize, n: usize) -> usize { if n == 0 { 0 } else { (self.next(p, side) % n as u64) as usize } }
    pub fn shuffle<T>(&mut self, p: Purpose, side: usize, v: &mut [T]) {
        for i in (1..v.len()).rev() { let j = self.below(p, side, i + 1); v.swap(i, j); }
    }
}
