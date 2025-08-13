use ratatui::style::{
    Style,
    Color
};
use ratatui::widgets::{
    Block, 
    Borders,
    List, ListItem,
    Paragraph
};
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::text::{Line, Span, Text};

use crate::app::{App, CurrentScreen, CurrentlyEditing};

/*
Widgets are constructed and then drawn onto the screen using a Frame, which is placed within a specified Rect
*/

/* UI function, creating UI elements */
/* Frame containes terminal data (like size) in render time */
/* App contatins the application data */
pub fn ui(frame: &mut Frame, app: &App) {
    // Creating main screen layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3)
        ])
        .split(frame.area()); // Cut the while frame rea into vertical pieces

    /*
        The variable chunks now contains a length 3 array of Rect objects 
        that contain the top left corner of their space, and their size.
        We will use these later, after we prepare our widgets.
     */

    ///// Top chunk /////

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "Create New JSON",
        Style::default().fg(Color::Green) //Set foreground color
    ))
    .block(title_block); // Surrounding the paragraph by the title_block
    frame.render_widget(title, chunks[0]); // Rendering the widget to the first chunk (top)

    ///// Middle chunk /////
    
    /* We would like to see the previous key-value pairs oon the gui */
    let mut list_items = Vec::<ListItem>::new();
    for key in app.pairs.keys() {
        list_items.push(ListItem::new(Line::from(Span::styled(
            format!("{: <25} : {}", key, app.pairs.get(key).unwrap()), //CONTENT
            Style::default().fg(Color::Yellow)  //STYLE
        ))));
    }

    let list = List::from_iter(list_items.into_iter()); //TODO tutorial does not works
    frame.render_widget(list, chunks[1]); // rendering the middle chunk


    ///// Middle Chunk /////
    
    /* Creating explanation, navigation helper text */
    let current_navigation_text = vec![
        // The first half of the text
        match app.current_screen {
            CurrentScreen::Main => {
                Span::styled("Normal Mode", Style::default().fg(Color::Green))
            }
            CurrentScreen::Editing => {
                Span::styled("Editing Mode", Style::default().fg(Color::Yellow)) 
            }
            CurrentScreen::Exiting => {
                Span::styled("Exiting", Style::default().fg(Color::LightRed))
            }
        }
        .to_owned()

        , /* Vector separator */

        //Adding a white divider bar to separate the two sections

        Span::styled(" | ", Style::default().fg(Color::White))

       , /* Vector separator */

        //Final section of the test, with hints on what the user is editing
        if let Some(editing) = &app.currently_editing {
            match editing {
                CurrentlyEditing::Key => {
                    Span::styled("Editing JSON key", Style::default().fg(Color::Green))
                }
                CurrentlyEditing::Value => {
                    Span::styled("Editing JSON value", Style::default().fg(Color::LightGreen))
                }
            }
        } else {
            Span::styled("Not Editing Anything", Style::default().fg(Color::DarkGray))
        }
    ];

    //Put it in a block to make it a widget
    let mode_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::ALL)); 

    //Adding a helping test for controls
    let current_keys_hint = {
        match app.current_screen {
            CurrentScreen::Main => Span::styled(
                "(q) to quit / (e) to make new pair",
                Style::default().fg(Color::Red)
            ),
            CurrentScreen::Editing => Span::styled(
                "(ESC) to cancel / (TAB) to switch boxes / (ENTER) to complete",
                Style::default().fg(Color::Red)
            ),
            CurrentScreen::Exiting => Span::styled(
                "(q) to quit / (e) to make new pair",
                Style::default().fg(Color::Red)
            )
        }
    };

    let key_notes_footer = Paragraph::new(Line::from(current_keys_hint))
        .block(Block::default().borders(Borders::ALL));

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage((50))])
        .split(chunks[2]); // Applying the Layout configuration to the bottom chunk

    frame.render_widget(mode_footer, footer_chunks[0]);
    frame.render_widget(key_notes_footer, footer_chunks[1]);


    

}

/// helper function to create a centered rect using up certain
/// percentage of the available rect `r`.
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y ),
            Constraint::Percentage((100 - percent_y) / 2)
        ])
        .split(r);

    // Cut the middle vertical piece into three width-wise pieces
    Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x ),
                Constraint::Percentage((100 - percent_x) / 2 )
            ])
            .split(popup_layout[1])[1] // return the middle chunk
}
