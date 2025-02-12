pub const MIDI_MIDDLE_C_NOTE: u8 = 60;
pub const MIDI_MIDDLE_C_OCTAVE: u8 = 4;

pub const MIDI_MSG_BUFFER_SIZE: usize = 16;

pub const NANOKEY2: &str = "nanoKEY2";

pub enum MidiStatus {
    NoteOn = 0b10010000,
}
