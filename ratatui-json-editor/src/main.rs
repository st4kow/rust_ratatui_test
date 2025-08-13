use std::{error::Error, io};

use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};

mod app;
mod ui;
use crate::{
    app::App,
    ui::ui
};

fn main() -> Result<(), Box<dyn Error>> {
    /*
    Setup terminal
    Here we are setting up stderr for rendering and stdout for printing.
    This way the useful output can be piperd into a text file from command line
    */

    enable_raw_mode()?;
    let mut stderr = io::stderr(); // standard error
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?; // set up crossterm to stderr


    /* Using the prepared custom terminal as the backand for ratatui */
    let backend = CrosstermBackend::new(stderr);
    let mut terminal= Terminal::new(backend)?;

    /* Create app and run it  */
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    /* Restore the terminal to the state we were starting with */
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;


    /* Check the result of the app and decide if printing needed */
    /* We do this after we went back to the great old terminal */
    if let Ok(do_print) = res {
        if do_print {
            app.print_json()?;
        }
    } else if let Err(err) = res {
        println!("{err:?}");
    }

    /* Return Ok */
    Ok(())
}

//TODO
fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;
    }

    io::Result::Ok((true))
}