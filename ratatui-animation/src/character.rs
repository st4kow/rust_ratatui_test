
use crate::{frame_data, position::Position};

#[derive(Default)]
pub struct Character {
    pub position: Position,
    pub speed: f64, // speed in units/event
    pub width: f64,
    pub height: f64,

    pub animating: bool, /* True, if animation in progress */
    anim_slice: u16, /* current animation slice */
    anim_position: Position,
    anim_speed: f64,
    anim_width: f64,
    anim_height: f64,
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

static ANIM_FRAME_SLICES: u16 = 10000;
static ANIM_TIME_MOVEMENT_TO_SMALL_MS: u32 = 200;
static ANIM_TIME_MOVEMENT_SHIFT_MS: u32 = 500;
static ANIM_TIME_MOVEMENT_TO_BIG_MS: u32 = 200;
static MOVEMENT_UNIT: u32 = 10;

impl Character {
    pub fn new() -> Character { 
        let mut character = Character::default();
        character.speed = 1.0;
        character.width = 5.0;
        character.height = 5.0;
        character.anim_type = AnimType::NONE;
        character
    }
    pub fn move_up(&mut self ) -> &mut Self {
        //self.position.decr_y(self.speed);
        self.anim_type = AnimType::UP;
        self
    }
    pub fn move_down(&mut self ) -> &mut Self {
        //self.position.incr_y(self.speed);
        self.anim_type = AnimType::DOWN;
        self
    }
    pub fn move_left(&mut self ) -> &mut Self {
        //self.position.decr_x(self.speed);
        self.anim_type = AnimType::LEFT;
        self
    }
    pub fn move_right(&mut self ) -> &mut Self {
        //self.position.incr_x(self.speed);
        self.anim_type = AnimType::RIGHT;
        self
    }
    pub fn animate(&mut self, scaler: f64) -> &mut Self {

        self.animating = true; /* TODO pre animation save */

        let full_anim_time = ANIM_TIME_MOVEMENT_SHIFT_MS;
        let anim_slice_time_us: f64 = full_anim_time as f64 / (ANIM_FRAME_SLICES as f64);
        let target_anim_slice_per_frame = (frame_data::TARGET_MSPS as f64 / anim_slice_time_us);
        let current_frame_num_slices = (target_anim_slice_per_frame.round() * scaler).round() as u16;

        let mut anim_slice_to_render: u16 = self.anim_slice + current_frame_num_slices;
        if anim_slice_to_render > ANIM_FRAME_SLICES { anim_slice_to_render = 0 };
        let delta_movement = ((MOVEMENT_UNIT as f64 / (ANIM_FRAME_SLICES as f64) ) * current_frame_num_slices as f64);
        match self.anim_type {
            AnimType::UP => {
                self.position.decr_y(delta_movement);
            },
            AnimType::DOWN => {
                self.position.incr_y(delta_movement);
            },
            AnimType::LEFT => {
                self.position.decr_x(delta_movement);
            },
            AnimType::RIGHT => {
                self.position.incr_x(delta_movement);
            },
            _ => {}
        }
        self.anim_slice = anim_slice_to_render;
        self
    }
}