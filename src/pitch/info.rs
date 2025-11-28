use std::fmt::{self, Display};

const A4_FREQUENCY: f64 = 440.0;

// Sharp symbol: ♯
// Flat symbol: ♭
const NOTE_LITERALS: [&str; 12] = [
    "A",
    "A♯/B♭",
    "B",
    "C",
    "C♯/D♭",
    "D",
    "D♯/E♭",
    "E",
    "F",
    "F♯/G♭",
    "G",
    "G♯/A♭",
];
const NUM_NOTES: i32 = NOTE_LITERALS.len() as i32;

fn get_index_based_on_semitone(semitone: i32) -> usize {
    let mut semitone_cycled = semitone % NUM_NOTES;

    if semitone_cycled < 0 {
        semitone_cycled += NUM_NOTES;
    }

    semitone_cycled as usize
}

fn get_octave_based_on_semitone(semitone: i32) -> u8 {
    // A4 as a semitone is 57...
    ((57 + semitone) / NUM_NOTES) as u8
}

#[derive(Debug, Clone)]
pub struct Note {
    pub literal: String,
    pub octave: u8,
}

impl Note {
    fn from_semitone(semitone: i32) -> Self {
        let index = get_index_based_on_semitone(semitone);

        Self {
            literal: String::from(NOTE_LITERALS[index]),
            octave: get_octave_based_on_semitone(semitone),
        }
    }
}

impl Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.literal, self.octave)
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
        let note = Note::from_semitone(semitones_rounded as i32);

        Self {
            note,
            cent: semitones - semitones_rounded,
        }
    }
}

fn get_semitones(frequency: f64) -> f64 {
    // Based on the formula F = 2^(n/12) * 440, where F is the frequency of the
    // note, and n is the number of semitones above A4. This is solved for n
    12.0 * (frequency / A4_FREQUENCY).log2()
}

pub fn get_pitch_info(frequency: f32) -> PitchInfo {
    let semitones = get_semitones(frequency as f64);
    PitchInfo::from_semitones(semitones)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pitch_info_test() {
        let pitch = get_pitch_info(440.0);
        assert_eq!(pitch.note.literal, "A");
        assert_eq!(pitch.note.octave, 4);

        let pitch = get_pitch_info(880.0);
        assert_eq!(pitch.note.literal, "A");
        assert_eq!(pitch.note.octave, 5);

        let pitch = get_pitch_info(660.0);
        assert_eq!(pitch.note.literal, "E");
        assert_eq!(pitch.note.octave, 5);

        let pitch = get_pitch_info(1320.0);
        assert_eq!(pitch.note.literal, "E");
        assert_eq!(pitch.note.octave, 6);

        let pitch = get_pitch_info(830.0);
        assert_eq!(pitch.note.literal, "G♯");
        assert_eq!(pitch.note.octave, 5);

        let pitch = get_pitch_info(460.0);
        assert_eq!(pitch.note.literal, "B♭");
        assert_eq!(pitch.note.octave, 4);
    }
}
