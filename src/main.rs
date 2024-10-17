use std::io::{self, stdout};

use midir::{Ignore, MidiInput, MidiOutput};

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

use sysinfo::{Components, Disks, Networks, System};

struct Midomo {
    system: System,
    midi_in: MidiInput,
    midi_out: MidiOutput,
}

impl Midomo {
    fn new() -> Self {
        let mut midomo = Midomo {
            system: System::new_all(),
            midi_in: MidiInput::new("midomo MIDI input").unwrap(),
            midi_out: MidiOutput::new("midomo MIDI output").unwrap(),
        };

        midomo.midi_in.ignore(Ignore::None);

        midomo.system.refresh_all();

        return midomo;
    }
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let midomo = Midomo::new();

    let mut should_quit = false;
    while !should_quit {
        terminal.draw(|frame| ui(frame, &midomo))?;
        should_quit = handle_events()?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn handle_events() -> io::Result<bool> {
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
    let mut midi_in_ports = String::new();

    for (i, p) in midomo.midi_in.ports().iter().enumerate() {
        midi_in_ports.push_str(&format!(
            "{}: {}\n",
            i,
            midomo.midi_in.port_name(p).unwrap()
        ));
    }

    frame.render_widget(
        Paragraph::new(midi_in_ports).block(Block::bordered().title("midomo")),
        frame.area(),
    );
}
