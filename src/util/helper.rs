use crate::util::constant::{MIDI_MIDDLE_C_NOTE, MIDI_MIDDLE_C_OCTAVE};

pub fn midi_note_to_string(midi_int: u8) -> Result<String, bool> {
    let pitch;
    let octave;

    let offset_from_middle_c = midi_int as i16 - MIDI_MIDDLE_C_NOTE as i16;

    let pitch_offset = offset_from_middle_c % 12;
    let octave_offset = offset_from_middle_c / 12;

    let mut pitch_offset_normalized = pitch_offset;

    if pitch_offset_normalized < 0 {
        while pitch_offset_normalized < 0 {
            pitch_offset_normalized += 12;
        }
    }

    match pitch_offset_normalized {
        0 => pitch = "C",
        1 => pitch = "C#",
        2 => pitch = "D",
        3 => pitch = "D#",
        4 => pitch = "E",
        5 => pitch = "F",
        6 => pitch = "F#",
        7 => pitch = "G",
        8 => pitch = "G#",
        9 => pitch = "A",
        10 => pitch = "A#",
        11 => pitch = "B",
        _ => return Result::Err(false),
    }

    match octave_offset {
        -16..=16 => octave = octave_offset + MIDI_MIDDLE_C_OCTAVE as i16,
        _ => return Result::Err(false),
    }

    let mut combined = String::new();
    combined.push_str(pitch);
    combined.push_str(&octave.to_string());

    return Result::Ok(combined);
}
