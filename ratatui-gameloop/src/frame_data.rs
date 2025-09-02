use std::time::{Duration, Instant};

pub static TARGET_FPS : u128 = 60; 
pub static TARGET_MSPS: u128 = ((1.0 / TARGET_FPS as f64) * 1000.0 ) as u128;
pub static TARGET_USPS: u128 = ((1.0 / TARGET_FPS as f64) * 1000.0 * 1000.0 ) as u128;

pub mod frame_data

pub fn update_last_frame()




struct FrameDataRaw {
    frame_number: u128,
    last_frame_time_us: u128,
    scale: f64,

    last_frame_timestamp: Instant,

}

impl FrameDataRaw {
    pub fn init() -> FrameDataRaw {
        FrameDataRaw {
            frame_number: (0),
            last_frame_time_us: (0),
            scale: (1.0),
            last_frame_timestamp: Instant::now()
        }
    }
}

