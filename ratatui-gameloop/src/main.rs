use std::{error::Error, io, thread, time};

use ratatui::{
    backend::{Backend},
    Terminal,
};

use std::time::{Duration, Instant};

mod app;
mod ui;
mod character;
mod position;
mod terminal;
mod frame_data;

use crate::{
    app::App, frame_data::FrameData, ui::ui
};

fn main() -> Result<(), Box<dyn Error>> {

    /* itit terminal for ui */
    let mut terminal = terminal::init_terminal()?;

    /* Create app and run it  */
    let mut app = App::new();
    run_app(&mut terminal, &mut app)?;

    /* Restore the terminal to the state we were starting with */
    terminal::restore_terminal(&mut terminal)?;

    println!("Last fram time: {}", app.last_frame_time);
    Ok(())
}


fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<(), Box<dyn Error>> {

    /* Define FrameData for future frame inforamtion tracting */
    let mut fd = FrameData::init();
    let mut stop = false;
    while ! stop {
        /* Update FrameData to provide information of last frame duration */
        fd.update();

        // Draw the UI
        terminal.draw(|f| ui(f, app))?;

        // Handling interaction
        stop = terminal::handle_inputs_experiment(app)?;

        // TEST TODO
        app.last_frame_time = fd.last_frame_time_us();

        // Sleep to meet requ
        thread::sleep(time::Duration::from_micros(fd.target_tsleep_us() as u64));
    }

    Ok(())
}