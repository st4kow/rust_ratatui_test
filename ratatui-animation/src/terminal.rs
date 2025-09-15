use std::{io, error::Error};
use ratatui::{crossterm};
use ratatui::{backend::CrosstermBackend,
    crossterm::{
        event::{DisableMouseCapture, EnableMouseCapture},
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen},
        
    },
    Terminal,
};


/// # Ititialize terinal
/// This function created a raw-mode crossterm backend for ratatui
pub fn init_terminal() -> Result< Terminal<CrosstermBackend<io::Stderr> >, Box<dyn Error>> {

    /*
    Setup terminal
    Here we are setting up stderr for rendering and stdout for printing.
    This way the useful output can be piperd into a text file from command line
    */

    crossterm::terminal::enable_raw_mode()?;
    let mut stderr = io::stderr(); // standard error
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?; // set up crossterm to stderr


    /* Using the prepared custom terminal as the backand for ratatui */
    let backend = CrosstermBackend::new(stderr);

    /* Return the initialized terminal */
    Ok(Terminal::new(backend)?)
}

/// # Restore terminal
/// This function restores the terinal to the standard one the application was started whith
pub fn restore_terminal(terminal:  &mut Terminal<CrosstermBackend<io::Stderr>>) -> Result<(), Box<dyn Error>> {
    /* Restore the terminal to the state we were starting with */
    crossterm::terminal::disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}