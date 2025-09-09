
use std::{error::Error, io};

use ratatui::style::{
    Style,
    Color
};
use ratatui::widgets::canvas::{
    Canvas, Rectangle
};
use ratatui::widgets::{
    Block, 
    Borders,
    Clear
};
use ratatui::{Frame};

use crate::app::{App};

/*
Widgets are constructed and then drawn onto the screen using a Frame, which is placed within a specified Rect
*/

/* UI function, creating UI elements */
/* Frame containes terminal data (like size) in render time */
/* App contatins the application data */
pub fn ui(frame: &mut Frame, app: &App) {
    /* Testin the game part */
    frame.render_widget(Clear, frame.area()); //Clear the whole screen
    let game_block = Block::default()
        .title("This is the main block")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::DarkGray));

    let rect_player = Rectangle {
        x: app.character.position.get_x(),
        y: app.character.position.get_y(),
        width: app.character.width,
        height: app.character.height,
        color: Color::Red
    };

    let game_canvas = Canvas::default()
        .block(game_block)
        .marker(ratatui::symbols::Marker::Braille)
        .x_bounds([0.0, 100.0]) //Scaling 0 - 100%
        .y_bounds([0.0, 100.0]) // Scaling 0 - 100%
        .paint(|ctx| {
            ctx.draw(&rect_player);
        })
    ;
    frame.render_widget(game_canvas, frame.area());

}

