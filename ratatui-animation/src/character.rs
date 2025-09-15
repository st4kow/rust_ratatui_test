
use crate::{frame_data, position::Position};

#[derive(Default)]
pub struct Character {
    pub position: Position, /* Position of the origin, in this case the middle of the character (rectangle) */
    pub speed: f64, // speed in units/event
    pub width: f64,
    pub height: f64,

    anim_slice: u32, /* current animation slice */

    anim_start_position: Position,
    anim_start_speed: f64,
    anim_start_width: f64,
    anim_start_height: f64,

    anim_target_position: Position,
    anim_target_speed: f64,
    anim_target_width: f64,
    anim_target_height: f64,

    anim_type: AnimType
}

#[derive(Default)]
#[derive(PartialEq)]
enum AnimType {
    #[default]
    NONE,
    UP,
    DOWN,
    LEFT,
    RIGHT
}

/* Animation pparameters */
const ANIM_FRAME_SLICES: u32 = 10000; 
const ANIM_TIME_MOVEMENT_TO_SMALL_MS: u32 = 200;
const ANIM_TIME_MOVEMENT_SHIFT_MS: u32 = 400;
const ANIM_TIME_MOVEMENT_TO_BIG_MS: u32 = 200;
const WIDTH_SMALL: u32 = 3; const HEIGHT_SMALL: u32 = 3;
const WIDTH_BIG: u32 = 10; const HEIGHT_BIG: u32 = 10;
const MOVEMENT_UNIT: u32 = WIDTH_BIG; /* Moving on a grid */

/* Deriving commulated time info for further use */
const ANIM_TIME_FULL_MS: u32 = ANIM_TIME_MOVEMENT_TO_SMALL_MS +
    ANIM_TIME_MOVEMENT_SHIFT_MS +
    ANIM_TIME_MOVEMENT_TO_BIG_MS;
const ANIM_SLICE_TIME_US: u32 = ((ANIM_TIME_FULL_MS as f64 / ANIM_FRAME_SLICES as f64) * 1000.0) as u32; // TODO rounding error

/* Describing slices for animation */
const SLICE_FROM_MOVEMENT_TO_SMALL: u32 = 0;
const SLICE_FROM_MOVEMENT_SHIFT: u32 = ANIM_TIME_MOVEMENT_TO_SMALL_MS as u32 * 1000 / ANIM_SLICE_TIME_US as u32
    +SLICE_FROM_MOVEMENT_TO_SMALL;
const SLICE_FROM_MOVEMENT_TO_BIG: u32 = ANIM_TIME_MOVEMENT_SHIFT_MS as u32 * 1000 / ANIM_SLICE_TIME_US as u32
    +SLICE_FROM_MOVEMENT_SHIFT;

impl Character {

    /*--------------------
    Public methods
    --------------------*/

    pub fn new() -> Character { 
        let mut character = Character::default();
        character.speed = 1.0;
        character.width = WIDTH_BIG as f64;
        character.height = HEIGHT_BIG as f64;
        character.anim_type = AnimType::NONE;

        /* Placing character at the bottom left corner */
        character.position.set_x(character.width/2.0); character.position.set_y(character.width/2.0);
        character
    }

    pub fn move_up(&mut self ) -> &mut Self {
        if !self.anim_in_progress() {
            self.anim_type = AnimType::UP;
            self.start_animation();
        }
        self
    }
    pub fn move_down(&mut self ) -> &mut Self {
        //self.position.incr_y(self.speed);
        if !self.anim_in_progress() {
            self.anim_type = AnimType::DOWN;
            self.start_animation();
        }
        self
    }
    pub fn move_left(&mut self ) -> &mut Self {
        //self.position.decr_x(self.speed);
        if !self.anim_in_progress() {
            self.anim_type = AnimType::LEFT;
            self.start_animation();
        }
        self
    }
    pub fn move_right(&mut self ) -> &mut Self {
        //self.position.incr_x(self.speed);
        if !self.anim_in_progress() {
            self.anim_type = AnimType::RIGHT;
            self.start_animation();
        }
        self
    }

    pub fn update(&mut self, scaler: f64) -> &mut Self {

        /* If no need to animate do not change anything */
        if !self.anim_in_progress() { return self; }

        let full_anim_time = ANIM_TIME_MOVEMENT_SHIFT_MS;
        let anim_slice_time_us: f64 = full_anim_time as f64 / (ANIM_FRAME_SLICES as f64);
        let target_anim_slice_per_frame = frame_data::TARGET_MSPS as f64 / anim_slice_time_us;
        let current_frame_num_slices = (target_anim_slice_per_frame.round() * scaler).round() as u32;

        let mut anim_slice_to_render: u32 = self.anim_slice + current_frame_num_slices;

        /* Overflow of animation slice counter, meaning we are ready */
        if anim_slice_to_render > ANIM_FRAME_SLICES { 
            anim_slice_to_render = ANIM_FRAME_SLICES /* Render the last slice */ 
        };

        if anim_slice_to_render >= SLICE_FROM_MOVEMENT_TO_BIG { /* Animation of transition back to big */

            /* 
                Setting everything that happened before this animation phase
            */

            /* Updating postion */
            self.position = self.anim_target_position.clone();

            /* Calculating phase of current animation section */
            let anim_phase = calc_anim_phase(SLICE_FROM_MOVEMENT_TO_BIG, ANIM_FRAME_SLICES, anim_slice_to_render);

            /* Updating width and height based on animation section phase */
            let width = (((WIDTH_BIG - WIDTH_SMALL) as f64 * anim_phase) + WIDTH_SMALL as f64)
                .round();
            let height = (((HEIGHT_BIG - HEIGHT_SMALL) as f64 * anim_phase) + HEIGHT_SMALL as f64)
                .round();
            self.width = width;
            self.height = height;

        } else if anim_slice_to_render >= SLICE_FROM_MOVEMENT_SHIFT { /* Animation of movement */

            /* 
                Setting everything that happened before this animation phase
            */

            /* Set width and height to small */
            self.width = WIDTH_SMALL as f64;
            self.height = WIDTH_BIG as f64;

            /* Calculating phase of current animation section */
            let anim_phase = calc_anim_phase(SLICE_FROM_MOVEMENT_SHIFT, SLICE_FROM_MOVEMENT_TO_BIG, anim_slice_to_render);
            
            /* Calculating movement based on animation section phase */
            let movement_offset = MOVEMENT_UNIT as f64 * anim_phase;
            match self.anim_type {
                AnimType::UP => {
                    self.position.set_y(self.anim_start_position.get_y() - movement_offset);
                },
                AnimType::DOWN => {
                    self.position.set_y(self.anim_start_position.get_y() + movement_offset);
                },
                AnimType::LEFT => {
                    self.position.set_x(self.anim_start_position.get_x() - movement_offset);
                },
                AnimType::RIGHT => {
                    self.position.set_x(self.anim_start_position.get_x() + movement_offset);
                },
                _ => {}
            }

        } else if anim_slice_to_render >= SLICE_FROM_MOVEMENT_TO_SMALL { /* Animation of transition to small */
            let anim_phase = calc_anim_phase(SLICE_FROM_MOVEMENT_TO_SMALL, SLICE_FROM_MOVEMENT_SHIFT, anim_slice_to_render);

            /* Updating width and height */
            let width = (((WIDTH_BIG - WIDTH_SMALL) as f64 * (1.0-anim_phase) ) + WIDTH_SMALL as f64)
                .round();
            let height = (((HEIGHT_BIG - HEIGHT_SMALL) as f64 * (1.0-anim_phase) ) + WIDTH_SMALL as f64)
                .round();
            self.width = width;
            self.height = height;
        }

        /* Update enimation slice so next iteration knows where we are */
        self.anim_slice = anim_slice_to_render;
        
        /* Stop the animation if last slice was rendered (animation was done) */
        if anim_slice_to_render == ANIM_FRAME_SLICES {
            self.anim_type = AnimType::NONE;
        }
        self
    }

    fn start_animation(&mut self) -> () {

        /* Reset current animation slice to 0 */
        self.anim_slice = 0;

        /* Update informmation at the the start of an animation */
        self.anim_start_height = self.height;
        self.anim_start_width = self.width;
        self.anim_start_speed = self.speed;
        self.anim_start_position.set_x(self.position.get_x());
        self.anim_start_position.set_y(self.position.get_y());
        self.anim_start_position.set_z(self.position.get_z());

        /* Update information at the end of the animation */
        self.anim_target_height = self.height;
        self.anim_target_width = self.width;
        self.anim_target_speed = self.speed;

        // This calculation is wrong TODO
        match self.anim_type {
            AnimType::UP => {
                self.anim_target_position.set_y(self.anim_start_position.get_y() - MOVEMENT_UNIT as f64);
            },
            AnimType::DOWN => {
                self.anim_target_position.set_y(self.anim_start_position.get_y() + MOVEMENT_UNIT as f64);
            },
            AnimType::LEFT => {
                self.anim_target_position.set_x(self.anim_start_position.get_x() - MOVEMENT_UNIT as f64);
            },
            AnimType::RIGHT => {
                self.anim_target_position.set_x(self.anim_start_position.get_x() + MOVEMENT_UNIT as f64);
            },
            _ => {}
        }

        ()
    }

    /*--------------------
    Helper methods
    --------------------*/

    fn anim_in_progress(&self) -> bool {
        self.anim_type != AnimType::NONE
    }


}

/*--------------------
Public functions
--------------------*/

//

/*--------------------
Helper functions
--------------------*/

fn calc_anim_phase(start: u32, end: u32, current: u32) -> f64 {
    (current - start) as f64 / (end - start) as f64
}