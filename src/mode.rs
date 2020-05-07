use crate::{Interval, SEMITONE, TONE};
use strum_macros::{Display as EnumDisplay, EnumIter, EnumString};

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
    type Item = Interval;
    type IntoIter = ModeIntervalIter;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            intervals: match self {
                Mode::Ionian => [TONE, TONE, SEMITONE, TONE, TONE, TONE],
                Mode::Dorian => [TONE, SEMITONE, TONE, TONE, TONE, SEMITONE],
                Mode::Phrygian => [SEMITONE, TONE, TONE, TONE, SEMITONE, TONE],
                Mode::Mixolydian => [TONE, TONE, TONE, SEMITONE, TONE, TONE],
                Mode::Lydian => [TONE, TONE, SEMITONE, TONE, TONE, SEMITONE],
                Mode::Aeolian => [TONE, SEMITONE, TONE, TONE, SEMITONE, TONE],
                Mode::Locrian => [SEMITONE, TONE, TONE, SEMITONE, TONE, TONE],
            },
            index: 0,
        }
    }
}

/// An iterator over a mode's intervals.
pub struct ModeIntervalIter {
    intervals: [Interval; 6],
    index: usize,
}

impl Iterator for ModeIntervalIter {
    type Item = Interval;

    fn next(&mut self) -> Option<Self::Item> {
        let interval = self.intervals.get(self.index).copied();
        self.index += 1;
        interval
    }
}
