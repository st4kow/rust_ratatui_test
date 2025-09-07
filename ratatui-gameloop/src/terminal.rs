use std::{io, error::Error};
use std::time::{Duration};
use ratatui::crossterm::event::KeyEvent;
use ratatui::{crossterm};
use ratatui::{ backend::CrosstermBackend,
    crossterm::{
        event::{self, Event, KeyCode, DisableMouseCapture, EnableMouseCapture},
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen},
        
    },
    Terminal,
};

use crate::app::App;


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

pub fn handle_inputs(app: &mut App) -> Result<bool, Box<dyn Error>> {

    // Wait 1ms for a new event. It there is no event, skip event for this frame
    let have_event: bool = event::poll(Duration::from_millis(0))?;
    if ! have_event { return Ok(false) };

    // event::read() is guranteed to be non blockking if have_event
    if let Event::Key(key) = event::read()? { 
        if key.kind == event::KeyEventKind::Release {
            return Ok((false));
        }
        match key.code {
            KeyCode::Left =>  { app.character.move_left(); }
            KeyCode::Right => { app.character.move_right(); }
            KeyCode::Up => { app.character.move_down(); } // TODO naming
            KeyCode::Down => { app.character.move_up(); } // TODO naming
            KeyCode::Char('q') => { return Ok(true); }
            _ => {}
        }
    }
    Ok(false)
}

pub fn handle_inputs_experiment(app: &mut App) -> Result<bool, Box<dyn Error>> {

    //let mut have_event: bool = false;
    let mut event: Option<Event> = None;
    while event::poll(Duration::from_millis(1))? {
        event = Some(event::read()?);
    }
    if let None = event { return Ok(false) };

    //If we get here, we can be sure that we have a valied event

    // event::read() is guranteed to be non blockking if have_event
    if let Some(Event::Key(key)) = event { 
        if key.kind == event::KeyEventKind::Release {
            return Ok((false));
        }
        match key.code {
            KeyCode::Left =>  { app.character.move_left(); }
            KeyCode::Right => { app.character.move_right(); }
            KeyCode::Up => { app.character.move_down(); } // TODO naming
            KeyCode::Down => { app.character.move_up(); } // TODO naming
            KeyCode::Char('q') => { return Ok(true); }
            _ => {}
        }
    }
    Ok(false)
}