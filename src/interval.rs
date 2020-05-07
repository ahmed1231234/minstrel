/// A semitone interval.
pub const SEMITONE: Interval = Interval::new(1);
/// A whole tone interval (two semitones)
pub const TONE: Interval = Interval::new(2);

/// A musical interval, represented as a number of semitones.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Interval {
    pub(crate) semitones: usize,
}

impl Interval {
    /// Creates a new `Interval` based on the given `semitones` value.
    pub const fn new(semitones: usize) -> Self {
        Self { semitones }
    }
}
