use color_eyre::eyre::Result;
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::{
    app::App,
    event::{Event, EventHandler},
    tui::Tui,
    update::update};

pub mod double_column_menu;
pub mod menus;
pub mod app;
pub mod event;
pub mod ui;
pub mod tui;
pub mod update;



fn main() -> Result<()> {
    // Create an application.
    let mut app = App::new()?;

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    // Start the main loop.
    while !app.should_quit {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => {}
            Event::Key(key_event) => update(&mut app, key_event),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        };
    }

    // Exit the user interface.
    tui.exit()?;

    Ok(())
}

