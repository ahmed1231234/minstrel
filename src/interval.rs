/// A semitone interval.
pub const SEMITONE: Interval = Interval::new(1);
/// A whole tone interval (two semitones).
pub const TONE: Interval = Interval::new(2);

/// An interval, represented as a number of semitones.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Interval {
    pub(crate) semitones: usize,
}

impl Interval {
    /// Creates a new `Interval` based on the given `semitones` value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use minstrel::Interval;
    ///
    /// let major_third = Interval::new(4);
    /// let minor_seventh = Interval::new(10);
    /// let octave = Interval::new(12);
    /// ```
    pub const fn new(semitones: usize) -> Self {
        Self { semitones }
    }
}
