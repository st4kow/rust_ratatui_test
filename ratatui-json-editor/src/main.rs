use std::{error::Error, io, time::Duration};

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
    app::{App, CurrentScreen, CurrentlyEditing},
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
        // Draw the UI
        terminal.draw(|f| ui(f, app))?;

        // Handling interaction
        // if event::polyy(Duration:form_millis(250))? { ... } // To prevent blocking of read
        if let Event::Key(key) = event::read()? { //read is bocking! TODO
            if key.kind == event::KeyEventKind::Release {
                continue; // Skip events that are not KeyEventKind::Press
            }
            match app.current_screen { // match for different screens
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('e') => {
                        app.current_screen = CurrentScreen::Editing;
                        app.currently_editing = Some(CurrentlyEditing::Key);
                    }
                    KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Exiting;
                    }
                    _ => {}
                },
                CurrentScreen::Exiting => match key.code {
                    KeyCode::Char('y') => {
                        return Ok(true);
                    }
                    KeyCode::Char('n') | KeyCode::Char('q') => {
                        return Ok(false);
                    }
                    _ => {}
                },
                CurrentScreen::Editing if key.kind == KeyEventKind::Press => {
                    match key.code {
                        KeyCode::Enter => {
                        if let Some(editing) = &app.currently_editing {
                            match editing {
                                CurrentlyEditing::Key => {
                                    app.currently_editing = Some(CurrentlyEditing::Value);
                                }
                                CurrentlyEditing::Value => {
                                    app.save_key_value();
                                    app.current_screen = CurrentScreen::Main;
                                    }
                                }
                            }
                        }
                        KeyCode::Backspace => {
                            if let Some(editing) = &app.currently_editing {
                                match editing {
                                    CurrentlyEditing::Key => {
                                        app.key_input.pop();
                                    }
                                    CurrentlyEditing::Value => {
                                        app.value_input.pop();
                                    }
                                }
                            }
                        }
                        KeyCode::Esc => {
                            app.current_screen = CurrentScreen::Main;
                            app.currently_editing = None;
                        }
                        KeyCode::Tab => {
                            app.toggle_editing();
                        }
                        KeyCode::Char(value) => {
                            if let Some(editing) = &app.currently_editing {
                                match editing {
                                    CurrentlyEditing::Key => {
                                        app.key_input.push(value);
                                    }
                                    CurrentlyEditing::Value => {
                                        app.value_input.push(value);
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
 
    }
}