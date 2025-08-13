use crossterm::event::{self, Event};
use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Paragraph},
    Frame,
};
fn main() {
    let mut terminal = ratatui::init();
    loop {
        terminal.draw(draw).expect("failed to draw frame");
        if matches!(event::read().expect("failed to read event"), Event::Key(_)) {
            break;
        }
    }
    ratatui::restore();
}

fn draw(frame: &mut Frame) {
    let areas = Layout::vertical([Constraint::Length(1); 4]).split(frame.area());

    let line = Line::from(vec![
        Span::raw("Hello "),
        Span::styled(
            "World",
            Style::new()
                .fg(Color::Green)
                .bg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        "!".red().on_light_yellow().italic(),
    ]);
    frame.render_widget(line, areas[0]);

    // using the short-hand syntax and implicit conversions
    let paragraph = Paragraph::new("Hello World!".red().on_white().bold());
    frame.render_widget(paragraph, areas[1]);

    // style the whole widget instead of just the text
    let paragraph = Paragraph::new("Hello World!").style(Style::new().red().on_white());
    frame.render_widget(paragraph, areas[2]);

    // use the simpler short-hand syntax
    let paragraph = Paragraph::new("Hello World!").blue().on_yellow();
    frame.render_widget(paragraph, areas[3]);
}