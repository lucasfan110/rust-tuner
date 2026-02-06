use anyhow::anyhow;
use std::{
    fmt::{self, Display},
    mem,
    str::FromStr,
};

const A4_FREQUENCY: f64 = 440.0;
const A4_AS_SEMITONE: i32 = 57;

// Sharp symbol: ♯
// Flat symbol: ♭
const NOTE_LITERALS: [&str; 12] = [
    "C",
    "C♯/D♭",
    "D",
    "D♯/E♭",
    "E",
    "F",
    "F♯/G♭",
    "G",
    "G♯/A♭",
    "A",
    "A♯/B♭",
    "B",
];
const NUM_NOTES: usize = NOTE_LITERALS.len();
const NUM_NOTES_I32: i32 = NUM_NOTES as i32;

const INVALID_NOTE_MESSAGE: &str = "Invalid note! Example note: Ab4, C#5, G6";

/// Kind of confusing, but basically if calculating the number of semitones above or below a4, the
/// number is based on A, with A being the starting point as 0 or any multiples of 12. But if shifting
/// to C, which is what the `NoteLiteral` enum is based on, then it needs to be shifted by -3
const NUM_SHIFT_BETWEEN_BASES: i32 = -3;

const fn get_octave_based_on_semitone(semitone: i32) -> i32 {
    (A4_AS_SEMITONE + semitone) / NUM_NOTES_I32
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
#[allow(unused)]
pub enum NoteLiteral {
    C = 0,
    CSharp,
    D,
    EFlat,
    E,
    F,
    FSharp,
    G,
    GSharp,
    A,
    BFlat,
    B,
}

impl NoteLiteral {
    fn from_usize(num: usize) -> Self {
        if !(0..NUM_NOTES).contains(&num) {
            panic!("Number does not exist in the NoteLiteral enum");
        }

        unsafe { mem::transmute(num) }
    }

    pub fn from_semitone_above_a4(semitone: i32) -> Self {
        let mut semitone_cycled = semitone % NUM_NOTES_I32;

        if semitone_cycled.is_negative() {
            semitone_cycled += NUM_NOTES_I32;
        }

        semitone_cycled += NUM_SHIFT_BETWEEN_BASES;

        if semitone_cycled.is_negative() {
            semitone_cycled += NUM_NOTES_I32;
        }

        Self::from_usize(semitone_cycled as usize)
    }

    const fn note_char_to_num(c: char) -> usize {
        let note = match c.to_ascii_uppercase() {
            'C' => Self::C,
            'D' => Self::D,
            'E' => Self::E,
            'F' => Self::F,
            'G' => Self::G,
            'A' => Self::A,
            'B' => Self::B,
            _ => Self::C,
        };

        note as usize
    }
}

impl FromStr for NoteLiteral {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() || s.len() > 2 {
            return Err(anyhow!(INVALID_NOTE_MESSAGE));
        }

        let chars: Vec<char> = s.chars().collect();

        let note = chars[0];

        if !matches!(note.to_ascii_uppercase(), 'A'..='G') {
            return Err(anyhow!(INVALID_NOTE_MESSAGE));
        }

        let mut note_num = Self::note_char_to_num(note);

        if let Some(accidental) = chars.get(1) {
            note_num = match accidental {
                '#' => (note_num + 1) % NUM_NOTES,
                'b' => (note_num + NUM_NOTES - 1) % NUM_NOTES,
                _ => return Err(anyhow!(INVALID_NOTE_MESSAGE)),
            }
        }

        Ok(Self::from_usize(note_num))
    }
}

#[derive(Debug, Clone)]
pub struct Note {
    pub literal: NoteLiteral,
    pub octave: i32,
}

impl Note {
    pub fn from_semitone_above_a4(semitone: i32) -> Self {
        Self {
            literal: NoteLiteral::from_semitone_above_a4(semitone),
            octave: get_octave_based_on_semitone(semitone),
        }
    }

    fn get_semitones_above_a4(&self) -> i32 {
        let literal_as_num = self.literal as i32;
        let semitones = self.octave * NUM_NOTES_I32 + literal_as_num;
        semitones - A4_AS_SEMITONE
    }

    pub fn get_pitch(&self) -> f64 {
        let semitones = self.get_semitones_above_a4() as f64;

        A4_FREQUENCY * 2.0f64.powf(semitones / NUM_NOTES as f64)
    }
}

impl Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", NOTE_LITERALS[self.literal as usize], self.octave)
    }
}

impl FromStr for Note {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<char> = s.chars().collect();

        let number_index = chars
            .iter()
            .position(|c| c.is_ascii_digit())
            .ok_or(anyhow!(INVALID_NOTE_MESSAGE))?;

        if s.len() - 1 > number_index {
            return Err(anyhow!(INVALID_NOTE_MESSAGE));
        }

        let octave = chars[number_index]
            .to_digit(10)
            .ok_or(anyhow!(INVALID_NOTE_MESSAGE))?;

        let note_literal = &s[0..number_index];

        Ok(Self {
            literal: NoteLiteral::from_str(note_literal)?,
            octave: octave as i32,
        })
    }
}

#[derive(Debug, Clone)]
pub struct PitchInfo {
    pub note: Note,
    pub cent: f64,
}

impl PitchInfo {
    fn from_semitones(semitones: f64) -> Self {
        let semitones_rounded = semitones.round();
        let note = Note::from_semitone_above_a4(semitones_rounded as i32);

        Self {
            note,
            cent: semitones - semitones_rounded,
        }
    }
}

fn get_semitones(frequency: f64) -> f64 {
    // Based on the formula F = 2^(n/12) * 440, where F is the frequency of the
    // note, and n is the number of semitones above A4. This is solved for n
    NUM_NOTES as f64 * (frequency / A4_FREQUENCY).log2()
}

pub fn get_pitch_info(frequency: f32) -> PitchInfo {
    let semitones = get_semitones(frequency as f64);
    PitchInfo::from_semitones(semitones)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_to_note_literal_test() -> anyhow::Result<()> {
        let note_literal = NoteLiteral::from_str("A#")?;
        assert_eq!(note_literal, NoteLiteral::BFlat);

        let note_literal = NoteLiteral::from_str("G")?;
        assert_eq!(note_literal, NoteLiteral::G);

        let note_literal = NoteLiteral::from_str("A")?;
        assert_eq!(note_literal, NoteLiteral::A);

        let note_literal = NoteLiteral::from_str("Ab")?;
        assert_eq!(note_literal, NoteLiteral::GSharp);

        let note_literal = NoteLiteral::from_str("c#")?;
        assert_eq!(note_literal, NoteLiteral::CSharp);

        let note_literal = NoteLiteral::from_str("Db")?;
        assert_eq!(note_literal, NoteLiteral::CSharp);

        let note_literal = NoteLiteral::from_str("fb")?;
        assert_eq!(note_literal, NoteLiteral::E);

        let note_literal = NoteLiteral::from_str("b#")?;
        assert_eq!(note_literal, NoteLiteral::C);

        assert!(NoteLiteral::from_str("AA#").is_err());
        assert!(NoteLiteral::from_str("what").is_err());
        assert!(NoteLiteral::from_str("").is_err());
        assert!(NoteLiteral::from_str("\n").is_err());
        assert!(NoteLiteral::from_str("Ub").is_err());
        assert!(NoteLiteral::from_str("h#").is_err());
        assert!(NoteLiteral::from_str("11").is_err());
        assert!(NoteLiteral::from_str("bt").is_err());

        Ok(())
    }

    #[test]
    fn string_to_note_test() -> anyhow::Result<()> {
        let note = Note::from_str("A4")?;
        assert_eq!(note.literal, NoteLiteral::A);
        assert_eq!(note.octave, 4);

        let note = Note::from_str("b#6")?;
        assert_eq!(note.literal, NoteLiteral::C);
        assert_eq!(note.octave, 6);

        let note = Note::from_str("Cb1")?;
        assert_eq!(note.literal, NoteLiteral::B);
        assert_eq!(note.octave, 1);

        let note = Note::from_str("A0")?;
        assert_eq!(note.literal, NoteLiteral::A);
        assert_eq!(note.octave, 0);

        let note = Note::from_str("C#9")?;
        assert_eq!(note.literal, NoteLiteral::CSharp);
        assert_eq!(note.octave, 9);

        assert!(Note::from_str("C#").is_err());
        assert!(Note::from_str("8").is_err());
        assert!(Note::from_str("").is_err());
        assert!(Note::from_str("way too many characters").is_err());
        assert!(Note::from_str("C#12").is_err());
        assert!(Note::from_str("4a").is_err());
        assert!(Note::from_str("C?2").is_err());

        Ok(())
    }
}
