use std::error;
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

/// # Call this function to handle user inputs, user events
/// 
pub fn handle_inputs_experiment(app: &mut App) -> Result<bool, Box<dyn Error>> {

    let events: Option<Vec<Event>> = collect_events()?;

    /* If we have no event, simple return */
    if let None = events { return Ok(false) } 

    /* From this point, it is sure that we have valid events */
    let events: Vec<Event> = events.unwrap();

    /* Do what needs to be done at specific events */
    for event in events.iter() {
        if let Event::Key(key) = event { 
            if key.kind == event::KeyEventKind::Release {
                return Ok(false);
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
    }
    Ok(false)
}

/// # Returns a vector containing the events in this frame
/// Return vector contains all the events that heppened during this frame
fn collect_events() -> Result<Option<Vec<Event>>, Box<dyn Error>> {
    let mut events: Vec<event::Event> = Vec::new();
    while event::poll(Duration::from_millis(0))? {
        let mut event = event::read()?;
        //unique_events.try_add(event);
        events.push(event);
    }

    if events.len() == 0 {
        return Ok(None);
    }

    Ok(Some(events))
}

/*
pub struct unique_events {
    event_list : Vec<event::Event>
}
impl unique_events {
    pub fn new() -> Self {
        unique_events { event_list: Vec::new() }
    }
    pub fn try_add(&mut self, event: event::Event) -> &mut Self {
        self.event_list.push(event);
        self
    }
    pub fn get_events(&self) -> &Vec<event::Event> {
        &self.event_list
    }
    pub fn clear(&mut self) -> &mut Self {
        self.event_list.clear();
        self
    }
}
*/