use crate::character::Character;

pub struct App {
    /* Struct to store data for UI and game logic */
    pub character: Character
}
impl App {
    pub fn new() -> App {
        App {
            character: Character::new()
        }
    }

    /****** HELPER FUNCTIONS ******/
    
}