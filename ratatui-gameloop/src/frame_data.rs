use std::time::{Instant};

pub static TARGET_FPS : u128 = 60; 
pub static TARGET_MSPS: u128 = ((1.0 / TARGET_FPS as f64) * 1000.0 ) as u128;
pub static TARGET_USPS: u128 = ((1.0 / TARGET_FPS as f64) * 1000.0 * 1000.0 ) as u128;

pub struct FrameData {
    frame_number: u128,
    last_frame_timestamp: Instant,
    last_frame_time_us: u128,
    scale: f64,
}

impl FrameData {

    /// # Initialize FrameData
    /// Use this function to construct a new FrameData object 
    pub fn init() -> FrameData {
        let mut fd = FrameData {
            frame_number: (0),
            last_frame_timestamp: Instant::now(),
            last_frame_time_us: (0),
            scale: (1.0)
        };

        fd.update();
        fd
        
    }

    /// # Updating last frame information
    /// Call this function in the main event loop to update scaling, time, frame
    /// parameters to support FPS independent operation
    pub fn update(&mut self) -> &Self {
        self.frame_number += 1;
        self.last_frame_time_us = self.last_frame_timestamp
            .elapsed()
            .as_micros();
        self.last_frame_timestamp = Instant::now();
        self.scale = (self.last_frame_time_us as f64 / TARGET_USPS as f64)
            .clamp(1.0, 100.0); // TODO decide max

        self
    }

    /// # Get the scaler
    /// Resturns the scaler for FPS indepentent quantity calculation
    pub fn get_scale(&self) -> &f64 {
        &self.scale
    }

    /// # Scale a quantity
    /// Returns the FPS independent scaled value of a f64 qunatity
    pub fn scale(&self, quantity : &f64) -> f64 {
        (*self.get_scale()) * (*quantity)
    }

    /// # Last Frame time in microseconds
    /// Returns the frame time duration before the last update call
    pub fn last_frame_time_us(&self) -> u128 {
        self.last_frame_time_us
    }

    // TODO not great here!
    /// # Target time to sleep to meet target FPS
    /// Return time to sleep
    pub fn target_tsleep_us(&self) -> u128 {
        (TARGET_USPS as i128 - self.last_frame_time_us as i128).clamp(0, 1000 * 1000) as u128 // TODO decide max
    }
}

