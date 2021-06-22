#[macro_use]
extern crate diesel;
extern crate dotenv;

use std::{error::Error, io, time::Duration};
use termion::{input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{backend::TermionBackend, Terminal};

mod events;
mod db_utils;
mod app;
mod components;
mod components_ui;
mod documents;
mod select_events;
mod input_buffer;
mod spell_search;

use events::{Config, Event, Events};
use app::{App, Component};

fn main() -> Result<(), Box<dyn Error>> {
    use log::LevelFilter;

    simple_logging::log_to_file("dnd-tui.log", LevelFilter::Info);
    let events = Events::with_config(Config {
        tick_rate: Duration::from_millis(200),
        ..Config::default()
    });

    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;


    let mut app = App::new("DND tui");
    loop {
        terminal.draw(|f| app.component_tree.draw(f, f.size()))?;

        match events.next()? {
            Event::Input(key) => {
                app.on_key(key);
            },
            _ => {}
        }
        if app.should_quit {
            break;
        }
    }

    Ok(())
}
