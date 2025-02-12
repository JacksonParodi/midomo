use crate::util::constant::{MidiStatus, MIDI_MSG_BUFFER_SIZE, NANOKEY2};
use crate::util::helper::midi_note_to_string;
use crate::RingBuffer;

use std::sync::{Arc, Mutex};

use midir::{Ignore, MidiInput, MidiInputConnection};

use enigo::{Enigo, Settings};

use sysinfo::System;

use super::message::MidiMessage;

pub struct MidomoData {
    system: System,
    enigo: Enigo,
    midi_input_connection: Option<MidiInputConnection<()>>,
    // active_connection: Option<MidiInputConnection<()>>,
    // last_msg: Arc<Mutex<[u8; 3]>>,
    buffer: Arc<Mutex<RingBuffer<MidiMessage>>>,
}

impl MidomoData {
    pub fn new() -> Self {
        let mut m_data = MidomoData {
            system: System::new_all(),
            enigo: Enigo::new(&Settings::default()).unwrap(),
            midi_input_connection: None,
            // active_connection: None,
            // last_msg: Arc::clone(&last_msg),
            buffer: Arc::new(Mutex::new(RingBuffer::new(MIDI_MSG_BUFFER_SIZE))),
        };

        m_data.system.refresh_all();

        let mut m_midi_in = MidiInput::new("midomo MIDI input").unwrap();
        m_midi_in.ignore(Ignore::None);

        let in_ports = m_midi_in.ports();

        let option_port = in_ports.into_iter().find(|port| {
            m_midi_in
                .port_name(port)
                .unwrap()
                .to_string()
                .contains(NANOKEY2)
        });

        let clone = Arc::clone(&m_data.buffer);

        m_data.midi_input_connection = Some(
            m_midi_in
                .connect(
                    &option_port.unwrap(),
                    "midomo MIDI input connection",
                    move |_timestamp, message, _| {
                        let status_byte = message[0];
                        let data_1_byte = message[1];
                        let data_2_byte = message[2];

                        let status_type = status_byte & 0b11110000;

                        match status_type {
                            // Note On message
                            val if val == MidiStatus::NoteOn as u8 => {
                                let pitch_number = data_1_byte;
                                let _velocity = data_2_byte;

                                let mut buffer = clone.lock().unwrap();
                                buffer.push(MidiMessage::new(MidiStatus::NoteOn, pitch_number));
                            }
                            _ => (),
                        }
                    },
                    (),
                )
                .expect("Could not connect to MIDI input"),
        );

        // let m_data = Arc::new(Mutex::new(m_data));
        return m_data;
    }

    pub fn get_enigo(&self) -> &Enigo {
        &self.enigo
    }
    pub fn get_buffer(&self) -> Arc<Mutex<RingBuffer<MidiMessage>>> {
        Arc::clone(&self.buffer)
    }
}
