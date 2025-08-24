use std::{error::Error, io};

use ratatui::{
    backend::{Backend},
    Terminal,
};

mod app;
mod ui;
mod character;
mod position;
mod terminal;

use crate::{
    app::{App},
    ui::ui
};

fn main() -> Result<(), Box<dyn Error>> {

    /* itit terminal for ui */
    let mut terminal = terminal::init_terminal()?;

    /* Create app and run it  */
    let mut app = App::new();
    run_app(&mut terminal, &mut app)?;

    /* Restore the terminal to the state we were starting with */
    terminal::restore_terminal(&mut terminal)?;

    Ok(())
}


fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<(), Box<dyn Error>> {
    let mut stop = false;
    while ! stop {
        // Draw the UI
        terminal.draw(|f| ui(f, app))?;

        // Handling interaction
        stop = terminal::handle_inputs(app)?;
    }
    
    Ok(())
}