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

struct Midomo {
    system: System,
    enigo: Enigo,
    midi_out: MidiOutput,
    active_connection: Option<MidiInputConnection<()>>,
    last_msg: Arc<Mutex<[u8; 3]>>,
}

impl Midomo {
    fn new() -> Self {
        let mut m_system = System::new_all();
        m_system.refresh_all();

        let m_midi_out = MidiOutput::new("midomo MIDI output").unwrap();

        let mut m_midi_in = MidiInput::new("midomo MIDI input").unwrap();
        let in_ports = m_midi_in.ports();
        let m_active_port = in_ports[0].clone();

        m_midi_in.ignore(Ignore::None);

        let last_msg = Arc::new(Mutex::new([0, 0, 0]));

        let mut midomo = Midomo {
            system: m_system,
            enigo: Enigo::new(&Settings::default()).unwrap(),
            midi_out: m_midi_out,
            active_connection: None,
            last_msg: Arc::clone(&last_msg),
        };

        let m_active_connection = m_midi_in
            .connect(
                &m_active_port,
                "midomo MIDI input connection",
                move |timestamp, message, _| {
                    // println!("{}: {:?} (len = {})", timestamp, message, message.len());
                    let mut last_msg = last_msg.lock().unwrap();
                    last_msg.copy_from_slice(&message);
                },
                (),
            )
            .unwrap();

        midomo.active_connection = Some(m_active_connection);

        return midomo;
    }
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut midomo = Midomo::new();

    let mut should_quit = false;
    while !should_quit {
        terminal.draw(|frame| ui(frame, &midomo))?;
        should_quit = handle_events(&mut midomo)?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn handle_events(midomo: &mut Midomo) -> io::Result<bool> {
    let last_msg = midomo.last_msg.lock().unwrap();

    if last_msg[0] == 144 {
        match last_msg[1] {
            60 => midomo.enigo.text("I pushed C").unwrap(),
            61 => midomo.enigo.text("I pushed C#").unwrap(),
            62 => midomo.enigo.text("I pushed D").unwrap(),
            63 => midomo.enigo.text("I pushed D#").unwrap(),
            64 => midomo.enigo.text("I pushed E").unwrap(),
            65 => midomo.enigo.text("I pushed F").unwrap(),
            66 => midomo.enigo.text("I pushed F#").unwrap(),
            67 => midomo.enigo.text("I pushed G").unwrap(),
            68 => midomo.enigo.text("I pushed G#").unwrap(),
            69 => midomo.enigo.text("I pushed A").unwrap(),
            70 => midomo.enigo.text("I pushed A#").unwrap(),
            71 => midomo.enigo.text("I pushed B").unwrap(),
            _ => (),
        }
        midomo.enigo.key(Key::Return, Press).unwrap();
    }

    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

fn ui(frame: &mut Frame, midomo: &Midomo) {
    let last_msg = midomo.last_msg.lock().unwrap();
    let display_text = format!("MIDI message : {:?}", last_msg);

    frame.render_widget(
        Paragraph::new(display_text).block(Block::bordered().title("midomo")),
        frame.area(),
    );
}
