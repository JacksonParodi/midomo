mod midomo;
mod util;

use midomo::buffer::RingBuffer;
use midomo::data::MidomoData;

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

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut m_data = MidomoData::new();

    let mut should_quit = false;
    while !should_quit {
        terminal.draw(|frame| ui(frame, &m_data))?;
        should_quit = handle_events(&mut m_data)?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn handle_events(m_data: &mut MidomoData) -> io::Result<bool> {
    let last_msg = m_data.last_msg.lock().unwrap();

    if last_msg[0] == 144 {
        match last_msg[1] {
            60 => m_data.enigo.text("I pushed C").unwrap(),
            61 => m_data.enigo.text("I pushed C#").unwrap(),
            62 => m_data.enigo.text("I pushed D").unwrap(),
            63 => m_data.enigo.text("I pushed D#").unwrap(),
            64 => m_data.enigo.text("I pushed E").unwrap(),
            65 => m_data.enigo.text("I pushed F").unwrap(),
            66 => m_data.enigo.text("I pushed F#").unwrap(),
            67 => m_data.enigo.text("I pushed G").unwrap(),
            68 => m_data.enigo.text("I pushed G#").unwrap(),
            69 => m_data.enigo.text("I pushed A").unwrap(),
            70 => m_data.enigo.text("I pushed A#").unwrap(),
            71 => m_data.enigo.text("I pushed B").unwrap(),
            _ => (),
        }
        m_data.enigo.key(Key::Return, Press).unwrap();
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

fn ui(frame: &mut Frame, m_data: &MidomoData) {
    let last_msg = m_data.last_msg.lock().unwrap();
    let display_text = format!("MIDI message : {:?}", last_msg);

    frame.render_widget(
        Paragraph::new(display_text).block(Block::bordered().title("midomo")),
        frame.area(),
    );
}
