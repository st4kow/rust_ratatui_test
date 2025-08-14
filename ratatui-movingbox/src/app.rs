use crate::character::Character;
use std::collections::HashMap;

/* Enum to store Application state (What the user is seeing right now in this case) */
pub enum CurrentScreen {
    Main,
    Editing,
    Exiting
}
/* When we are editing this enum stores if we are editing the Key or the Value */
/* This need to be stored, becasue rendering changes based on this information */
pub enum CurrentlyEditing {
    Key,
    Value
}


pub struct App {
    pub key_input: String, /* the currently being edited json key */
    pub value_input: String, /* the currently being edited json value */
    pub pairs: HashMap<String, String>, /* Representation of out key and value pairs */
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<CurrentlyEditing>, /* Optional state containing key / value editing info */

    pub character: Character
}
impl App {
    pub fn new() -> App {
        App {
            key_input: String::new(),
            value_input: String::new(),
            pairs: HashMap::new(),
            current_screen: CurrentScreen::Main,
            currently_editing: None,

            character: Character::new()
        }
    }

    /****** HELPER FUNCTIONS ******/
    
    pub fn save_key_value(&mut self) {
        self.pairs
            .insert(self.key_input.clone(), self.value_input.clone() );
        self.key_input = String::new();
        self.value_input = String::new();
        self.currently_editing = None;
    }
    pub fn toggle_editing(&mut self) {
        if let Some(edit_mode) = &self.currently_editing {
            match edit_mode {
                CurrentlyEditing::Key => self.currently_editing = Some(CurrentlyEditing::Value),
                CurrentlyEditing::Value => self.currently_editing = Some(CurrentlyEditing::Key)
            };
        } else { /* From None we are forced to edit the Key */
            self.currently_editing = Some(CurrentlyEditing::Key);
        }
    }
    pub fn print_json(&self) -> serde_json::Result<()> {
        let output = serde_json::to_string(&self.pairs)?;
        println!("{output}");
        Ok(())
    }
}