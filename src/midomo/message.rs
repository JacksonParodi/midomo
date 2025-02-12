use crate::util::{constant::MidiStatus, helper::midi_note_to_string};

pub struct MidiMessage {
    status: MidiStatus,
    pitch: u8,
}

impl MidiMessage {
    pub fn new(status: MidiStatus, pitch: u8) -> Self {
        MidiMessage { status, pitch }
    }

    pub fn to_string(&self) -> String {
        match midi_note_to_string(self.pitch) {
            Ok(s) => s,
            Err(_) => String::from("Error"),
        }
    }
}
