use std::io;

use crossterm::event:: {self, Event, KeyCode, KeyEvent, KeyEventKind};

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{canvas::{Canvas, Shape}, Block, Paragraph, Widget},
    DefaultTerminal, Frame
};

fn main() -> io::Result<()> {
    /* Initializing the default terminal */
    let mut terminal = ratatui::init();

    /* Usually this would be in a loop, running the applicaiton */
    let app_result = App::default().run(&mut terminal);
    
    /* When GUI ended, restore the terminal */
    ratatui::restore();

    /* Return the result */
    app_result
}

#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    exit: bool
}
impl App {

    /* Runs the application's main loop until the user quits */
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw( |frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }
    
    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area() ); 
        /* Rendering self works, because we implemented *Widget* for self */
    }

    // This function blocks!!! Unit there is an event from crossterm.
    // Better approach to use event::poll
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.decrement_counter(),
            KeyCode::Right => self.increment_counter(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn decrement_counter(&mut self) {
        self.counter -= 1;
    }

    fn increment_counter(&mut self) {
        self.counter += 1;
    }



}
/* Widget is a high level renderable object
   Widget impelements .render() which is used by the terminal.draw internally to render the thing
   Here we explain how to render out application */
/*
    *Rect* is a ratatui area type. We are rendering to a rectangle
    *Buffer* is a frame buffer. Widgets are interacting with this,
    And the terminal typre rendees based on the buffer content
*/
impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        /* Line is a text type object, consisting of *Spans* */
        let title = Line::from(" Counter App Tutorial ".bold());

        /* Creating a line from more spans */
        let instructions = Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);

        /* Block is a widget, usully used as a wrapper around lower level ones */
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        /* *Text* represents one or more *Lines* of texts */
        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            self.counter.to_string().yellow()
        ])]);

        /* Paragraph is awidget to display text */
        Paragraph::new(counter_text)
            .centered()
            .block(block) /* Surrounding the Paragraph by a block */
            .render(area, buf);



        /* EXPERIMENTS FROM HERE
        let mut rectangle_test = ratatui::widgets::canvas::Rectangle::default();
        let mut canvas = Canvas::default()
            .block(Block::bordered().title("Experimet canvas block title"))
            .marker(ratatui::symbols::Marker::Braille)
            .x_bounds([10.0, 20.0])
            .y_bounds([10.0, 20.0])
            .paint(|cfx| {
                cfx.draw(&rectangle_test);
            })
            .render(area, buf);
        */
        
        
    }
}

/*
   -----
   TESTS
   -----
 */

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::style::Style;

    #[test]
    fn render() {
        let app = App::default();
        let mut buf = Buffer::empty(Rect::new(0, 0, 50, 4));

        app.render(buf.area, &mut buf);

        let mut expected = Buffer::with_lines(vec![
            "┏━━━━━━━━━━━━━ Counter App Tutorial ━━━━━━━━━━━━━┓",
            "┃                    Value: 0                    ┃",
            "┃                                                ┃",
            "┗━ Decrement <Left> Increment <Right> Quit <Q> ━━┛",
        ]);
        let title_style = Style::new().bold();
        let counter_style = Style::new().yellow();
        let key_style = Style::new().blue().bold();
        expected.set_style(Rect::new(14, 0, 22, 1), title_style);
        expected.set_style(Rect::new(28, 1, 1, 1), counter_style);
        expected.set_style(Rect::new(13, 3, 6, 1), key_style);
        expected.set_style(Rect::new(30, 3, 7, 1), key_style);
        expected.set_style(Rect::new(43, 3, 4, 1), key_style);

        assert_eq!(buf, expected);
    }
}

