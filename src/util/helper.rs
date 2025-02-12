use crate::util::constant::MIDI_MIDDLE_C;

pub fn midi_int_to_string(midi_int: u8) -> Result<String, bool> {
    let mut pitch = "X";
    let mut octave = -16;

    let mut combined = String::new();
    combined.push_str(pitch);
    combined.push_str(&octave.to_string());

    if &combined == "X-16" {
        return Result::Err(false);
    } else {
        return Result::Ok(combined);
    }
}
