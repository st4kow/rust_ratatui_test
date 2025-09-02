use crate::character::Character;

pub struct App {
    /* Struct to store data for UI and game logic */
    pub character: Character,
    pub last_frame_time: u128
}
impl App {
    pub fn new() -> App {
        App {
            character: Character::new(),
            frame_data: FrameData::new(),
            last_frame_time: 0u128
        }
    }

    /****** HELPER FUNCTIONS ******/
    
}