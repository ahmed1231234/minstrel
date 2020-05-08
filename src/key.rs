use crate::{Mode, Note};
use arrayvec::ArrayVec;
use std::fmt;
use strum::IntoEnumIterator;

/// A collection of seven notes, categorised by a `Mode`.
#[derive(Copy, Clone)]
pub struct Key {
    notes: [Note; 7],
    mode: Mode,
}

impl Key {
    /// Creates a new `Key` based on the given `root_note` and `mode`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use minstrel::{Key, Mode, Note};
    ///
    /// let c_major = Key::new(Note::new(0), Mode::Ionian);
    /// let a_minor = Key::new(Note::new(9), Mode::Aeolian);
    /// ```
    pub fn new(root_note: Note, mode: Mode) -> Self {
        let mut notes = ArrayVec::<[Note; 7]>::new();
        notes.push(root_note);
        // Procedurally creates the key based on the intervals
        // of the `mode`
        for (i, interval) in mode.into_iter().enumerate() {
            let previous_note = notes[i];
            notes.push(previous_note + *interval);
        }

        Self {
            notes: notes.into_inner().unwrap(),
            mode,
        }
    }

    /// Returns the key's seven notes without regard for octave numbers.
    ///
    /// See `Note::disregard_octave()` for more detail.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use minstrel::{Key, Mode, Note};
    ///
    /// let e_phrygian = Key::new(Note::new(4), Mode::Phrygian);
    /// assert_eq!(
    ///     e_phrygian.notes_disregarding_octave(),
    ///     [
    ///         Note::new(4),
    ///         Note::new(5),
    ///         Note::new(7),
    ///         Note::new(9),
    ///         Note::new(11),
    ///         Note::new(0),
    ///         Note::new(2),
    ///     ]
    /// );
    /// ```
    pub fn notes_disregarding_octave(mut self) -> [Note; 7] {
        for note in &mut self.notes {
            *note = note.disregard_octave();
        }
        self.notes
    }
}

#[cfg(test)]
#[test]
fn construction() {
    assert_eq!(
        Key::new(Note::new(0), Mode::Ionian).notes,
        [
            Note::new(0),
            Note::new(2),
            Note::new(4),
            Note::new(5),
            Note::new(7),
            Note::new(9),
            Note::new(11)
        ]
    );

    assert_eq!(
        Key::new(Note::new(3), Mode::Aeolian).notes,
        [
            Note::new(3),
            Note::new(5),
            Note::new(6),
            Note::new(8),
            Note::new(10),
            Note::new(11),
            Note::new(13)
        ]
    );
}

#[cfg(test)]
#[test]
fn octave_disregard() {
    assert_eq!(
        Key::new(Note::new(5), Mode::Ionian).notes_disregarding_octave(),
        [
            Note::new(5),
            Note::new(7),
            Note::new(9),
            Note::new(10),
            Note::new(0),
            Note::new(2),
            Note::new(4)
        ]
    );
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            let num_notes = self.notes.len();
            for (i, note) in self.notes.iter().enumerate() {
                write!(f, "{}", note)?;
                // Only prints a space if this was not the final note
                // (to avoid having a trailing space)
                if i != num_notes - 1 {
                    f.write_str(" ")?;
                }
            }

            Ok(())
        } else {
            write!(f, "{} {}", self.notes[0], self.mode)
        }
    }
}

#[cfg(test)]
mod display_tests {
    use super::*;

    #[test]
    fn normal() {
        assert_eq!(Key::new(Note::new(2), Mode::Dorian).to_string(), "D Dorian");
        assert_eq!(
            Key::new(Note::new(8), Mode::Mixolydian).to_string(),
            "Ab Mixolydian"
        );
    }

    #[test]
    fn alternate() {
        assert_eq!(
            format!("{:#}", Key::new(Note::new(2), Mode::Dorian)),
            "D E F G A B C"
        );

        assert_eq!(
            format!("{:#}", Key::new(Note::new(8), Mode::Mixolydian)),
            "Ab Bb C D Eb F G"
        );
    }
}

/// Guesses keys that the given collection of `notes` could belong to.
pub fn guess_key(notes: Vec<Note>, root_note: Option<Note>) -> Vec<Key> {
    // Loops through each mode for the given root note and checks if it
    // contains all of the given `notes`
    let key_filter = |root_note, key_candidates: &mut Vec<_>| {
        for mode in Mode::iter() {
            let key = Key::new(root_note, mode);
            if notes
                .iter()
                .all(|note| key.notes_disregarding_octave().contains(note))
            {
                key_candidates.push(key);
            }
        }
    };

    let mut key_candidates = Vec::new();
    // Uses the given `root_note` if it was supplied
    if let Some(root_note) = root_note {
        key_filter(root_note, &mut key_candidates);
    } else {
        for root_note in Note::new(0).into_iter().take(12) {
            key_filter(root_note, &mut key_candidates);
        }
    }
    key_candidates
}
