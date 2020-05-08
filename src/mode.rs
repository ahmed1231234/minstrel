use strum_macros::{Display as EnumDisplay, EnumIter, EnumString};

/// An enumeration over the seven musical modes.
#[derive(EnumString, EnumIter, EnumDisplay, Debug, Copy, Clone)]
pub enum Mode {
    Ionian,
    Dorian,
    Phrygian,
    Mixolydian,
    Lydian,
    Aeolian,
    Locrian,
}

impl IntoIterator for Mode {
    type Item = &'static usize;
    type IntoIter = std::slice::Iter<'static, usize>;

    fn into_iter(self) -> Self::IntoIter {
        // The seventh interval is ignored, since it simply returns to the octave
        // (which is already stored as the root note in a `Key`)
        match self {
            Mode::Ionian => [2, 2, 1, 2, 2, 2].iter(),
            Mode::Dorian => [2, 1, 2, 2, 2, 1].iter(),
            Mode::Phrygian => [1, 2, 2, 2, 1, 2].iter(),
            Mode::Mixolydian => [2, 2, 2, 1, 2, 2].iter(),
            Mode::Lydian => [2, 2, 1, 2, 2, 1].iter(),
            Mode::Aeolian => [2, 1, 2, 2, 1, 2].iter(),
            Mode::Locrian => [1, 2, 2, 1, 2, 2].iter(),
        }
    }
}
