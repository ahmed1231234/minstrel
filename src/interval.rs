pub const SEMITONE: Interval = Interval::new(1);
pub const TONE: Interval = Interval::new(2);

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Interval {
    pub semitones: usize,
}

impl Interval {
    pub const fn new(semitones: usize) -> Self {
        Self { semitones }
    }
}
