use std::io::{self, stdout};

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, Event, KeyCode},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    widgets::{Block, Paragraph},
    Frame, Terminal,
};

use sysinfo::{Components, Disks, Networks, System};

struct Midomo {
    system: System,
}

impl Midomo {
    fn new() -> Self {
        Midomo {
            system: System::new_all(),
        }
    }
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut midomo = Midomo::new();
    midomo.system.refresh_all();

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
    frame.render_widget(
        Paragraph::new(midomo.system.used_memory().to_string())
            .block(Block::bordered().title("Greeting")),
        frame.area(),
    );
}
