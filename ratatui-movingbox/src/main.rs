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
mod character;
mod position;

use crate::{
    app::{App},
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


    /* Check the result of the app and decide what to do */
    /* We do this after we went back to the great old terminal */
    if let Ok(do_print) = res {
        if do_print {
            // todo();
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
        // Draw the UI
        terminal.draw(|f| ui(f, app))?;

        // Handling interaction
        // if event::polyy(Duration:form_millis(250))? { ... } // To prevent blocking of read
        if let Event::Key(key) = event::read()? { //read is bocking! TODO
            if key.kind == event::KeyEventKind::Release {
                continue; // Skip events that are not KeyEventKind::Press
            }
            match key.code {
                KeyCode::Left =>  { app.character.move_left(); }
                KeyCode::Right => { app.character.move_right(); }
                KeyCode::Up => { app.character.move_down(); } // TODO naming
                KeyCode::Down => { app.character.move_up(); } // TODO naming
                KeyCode::Char('q') => { return Ok(false); }
                _ => {}
            }
        }
    }
}