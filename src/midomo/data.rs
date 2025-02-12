use crate::util;
use crate::RingBuffer;

use std::io::{self, stdout};
use std::sync::{Arc, Mutex};

use midir::{Ignore, MidiInput, MidiInputConnection, MidiInputPort, MidiOutput};

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, Event, KeyCode},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    style::{Style, Stylize},
    widgets::{Block, Paragraph},
    Frame, Terminal,
};

use enigo::{
    Button, Coordinate,
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Mouse, Settings,
};

use sysinfo::{Components, Disks, Networks, System};

pub struct MidomoData {
    system: System,
    pub enigo: Enigo,
    midi_out: MidiOutput,
    active_connection: Option<MidiInputConnection<()>>,
    // last_msg: Arc<Mutex<[u8; 3]>>,
    msg_buffer: RingBuffer<[u8; 3]>,
}

impl MidomoData {
    pub fn new() -> Self {
        let mut m_system = System::new_all();
        m_system.refresh_all();

        let m_midi_out = MidiOutput::new("midomo MIDI output").unwrap();

        let mut m_midi_in = MidiInput::new("midomo MIDI input").unwrap();
        let in_ports = m_midi_in.ports();
        let m_active_port = in_ports[0].clone();

        m_midi_in.ignore(Ignore::None);

        let last_msg = Arc::new(Mutex::new([0, 0, 0]));

        let mut midomo = MidomoData {
            system: m_system,
            enigo: Enigo::new(&Settings::default()).unwrap(),
            midi_out: m_midi_out,
            active_connection: None,
            // last_msg: Arc::clone(&last_msg),
            msg_buffer: RingBuffer::new(util::constant::MIDI_MSG_BUFFER_SIZE),
        };

        let m_active_connection = m_midi_in
            .connect(
                &m_active_port,
                "midomo MIDI input connection",
                move |timestamp, message, _| {
                    // println!("{}: {:?} (len = {})", timestamp, message, message.len());
                    // let mut last_msg = last_msg.lock().unwrap();
                    // last_msg.copy_from_slice(&message);
                },
                (),
            )
            .unwrap();

        midomo.active_connection = Some(m_active_connection);

        return midomo;
    }
}
