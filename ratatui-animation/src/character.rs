
use crate::{frame_data, position::Position};

#[derive(Default)]
pub struct Character {
    pub position: Position,
    pub speed: f64, // speed in units/event
    pub width: f64,
    pub height: f64,

    pub animating: bool, /* True, if animation in progress */
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
enum AnimType {
    #[default]
    NONE,
    UP,
    DOWN,
    LEFT,
    RIGHT
}

const ANIM_FRAME_SLICES: u32 = 10000;
const ANIM_TIME_MOVEMENT_TO_SMALL_MS: u16 = 200;
const ANIM_TIME_MOVEMENT_SHIFT_MS: u16 = 200;
const ANIM_TIME_MOVEMENT_TO_BIG_MS: u16 = 200;
const MOVEMENT_UNIT: u16 = 9;

/* Deriving commulated time info for further use */
const ANIM_TIME_FULL_MS: u16 = ANIM_TIME_MOVEMENT_TO_SMALL_MS +
    ANIM_TIME_MOVEMENT_SHIFT_MS +
    ANIM_TIME_MOVEMENT_TO_BIG_MS;
const ANIM_SLICE_TIME_US: u16 = ((ANIM_TIME_FULL_MS as f64 / ANIM_FRAME_SLICES as f64) * 1000.0) as u16; // TODO rounding error

/* Describing slices for animation */
const SLICE_FROM_MOVEMENT_TO_SMALL: u32 = 0;
const SLICE_FROM_MOVEMENT_SHIFT: u32 = ANIM_TIME_MOVEMENT_TO_SMALL_MS as u32 * 1000 / ANIM_SLICE_TIME_US as u32
    +SLICE_FROM_MOVEMENT_TO_SMALL;
const SLICE_FROM_MOVEMENT_TO_BIG: u32 = ANIM_TIME_MOVEMENT_TO_BIG_MS as u32 * 1000 / ANIM_SLICE_TIME_US as u32
    +SLICE_FROM_MOVEMENT_SHIFT;

const WIDTH_SMALL: u32 = 3; const HEIGHT_SMALL: u32 = 3;
const WIDTH_BIG: u32 = 6; const HEIGHT_BIG: u32 = 6;


impl Character {
    pub fn new() -> Character { 
        let mut character = Character::default();
        character.speed = 1.0;
        character.width = 5.0;
        character.height = 5.0;
        character.anim_type = AnimType::NONE;
        character.position.set_x(30.0); character.position.set_y(30.0);
        character
    }
    pub fn move_up(&mut self ) -> &mut Self {
        //self.position.decr_y(self.speed);
        if !self.animating {
            self.anim_type = AnimType::UP;
            self.start_animation();
        }
        self
    }
    pub fn move_down(&mut self ) -> &mut Self {
        //self.position.incr_y(self.speed);
        if !self.animating {
            self.anim_type = AnimType::DOWN;
            self.start_animation();
        }
        self
    }
    pub fn move_left(&mut self ) -> &mut Self {
        //self.position.decr_x(self.speed);
        if !self.animating {
            self.anim_type = AnimType::LEFT;
            self.start_animation();
        }
        self
    }
    pub fn move_right(&mut self ) -> &mut Self {
        //self.position.incr_x(self.speed);
        if !self.animating {
            self.anim_type = AnimType::RIGHT;
            self.start_animation();
        }
        self
    }

    pub fn update(&mut self, scaler: f64) -> &mut Self {

        /* If no need to animate do not change anything */
        if !self.animating { return self; }

        let full_anim_time = ANIM_TIME_MOVEMENT_SHIFT_MS;
        let anim_slice_time_us: f64 = full_anim_time as f64 / (ANIM_FRAME_SLICES as f64);
        let target_anim_slice_per_frame = (frame_data::TARGET_MSPS as f64 / anim_slice_time_us);
        let current_frame_num_slices = (target_anim_slice_per_frame.round() * scaler).round() as u32;

        let mut anim_slice_to_render: u32 = self.anim_slice + current_frame_num_slices;

        /* Overflow of animation slice counter, meaning we are ready */
        if anim_slice_to_render > ANIM_FRAME_SLICES { anim_slice_to_render = 0 };

        if anim_slice_to_render >= SLICE_FROM_MOVEMENT_TO_BIG { /* Animation of transition back to big */
            let anim_phase = calc_anim_phase(SLICE_FROM_MOVEMENT_TO_BIG, ANIM_FRAME_SLICES, anim_slice_to_render);

            /* Updating width and height */
            let width = (((WIDTH_BIG - WIDTH_SMALL) as f64 * anim_phase) + WIDTH_SMALL as f64)
                .round();
            let height = (((HEIGHT_BIG - HEIGHT_SMALL) as f64 * anim_phase) + HEIGHT_SMALL as f64)
                .round();
            self.width = width;
            self.height = height;

            /* Updating vertical and horizontal offset */
            let horizontal_offset = ((WIDTH_BIG - WIDTH_SMALL) as f64 * anim_phase) / 2.0;
            let vertical_offset = ((HEIGHT_BIG - HEIGHT_SMALL) as f64 * anim_phase) / 2.0;
            //self.position.set_x(self.anim_target_position.get_x() - horizontal_offset);
            //self.position.set_y(self.anim_target_position.get_y() + vertical_offset);


        } else if anim_slice_to_render >= SLICE_FROM_MOVEMENT_SHIFT { /* Animation of movement */
            let anim_phase = calc_anim_phase(SLICE_FROM_MOVEMENT_SHIFT, SLICE_FROM_MOVEMENT_TO_BIG, anim_slice_to_render);
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

            /* Updating vertical and horizontal offset */ // TODO Wrong, becasue 0 frame runs again at the end, but target position not updated
            let horizontal_offset = ((WIDTH_BIG - WIDTH_SMALL) as f64 * (1.0-anim_phase)) / 2.0;
            let vertical_offset = ((HEIGHT_BIG - HEIGHT_SMALL) as f64 * (1.0-anim_phase)) / 2.0;
            //self.position.set_x(self.anim_start_position.get_x() + horizontal_offset);
            //self.position.set_y(self.anim_start_position.get_y() - vertical_offset);
        }

        /* Update enimation slice for next render */
        self.anim_slice = anim_slice_to_render;
        
        /* Set animation to false if slice 0 was rendered (animation was done) */
        // TODO possible issue, if the first iteration is faster than 1 animation slice
        if anim_slice_to_render == 0 {
            self.animating = false;
            self.anim_type = AnimType::NONE;
        }
        self
    }

    fn start_animation(&mut self) -> () {

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

        /* Set the animation flag true to indicate animation is in progress */
        self.animating = true;

        ()
    }

}

fn calc_anim_phase(start: u32, end: u32, current: u32) -> f64 {
    (current - start) as f64 / (end - start) as f64
}