use std::{error::Error};
use std::time::{Duration};
use ratatui::crossterm::{
        event::{self, Event, KeyCode},
        
    };

use crate::app::App;

/// # Call this function to handle user inputs, user events
/// 
pub fn handle_inputs(app: &mut App) -> Result<bool, Box<dyn Error>> {

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
        let event = event::read()?;
        events.push(event);
    }

    if events.len() == 0 {
        return Ok(None);
    }

    Ok(Some(events))
}