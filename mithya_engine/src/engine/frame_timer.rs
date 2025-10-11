// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::time::Instant;

// Helper struct for FPS calculation
pub struct FrameTimer {
    last_frame_time: Instant,
    last_fps_time: Instant,
    frame_count: u32,
    fps: f32,
    delta_time: f32,
}

impl FrameTimer {
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            last_frame_time: now,
            last_fps_time: now,
            frame_count: 0,
            fps: 0.0,
            delta_time: 0.0,
        }
    }
    
    pub fn update(&mut self) {
        let current_time = Instant::now();
        
        // Calculate delta time from last frame
        self.delta_time = current_time.duration_since(self.last_frame_time).as_secs_f32();
        self.last_frame_time = current_time;
        
        // Update FPS counter
        self.frame_count += 1;
        let fps_elapsed = current_time.duration_since(self.last_fps_time);
        
        if fps_elapsed.as_secs_f32() >= 1.0 {
            self.fps = self.frame_count as f32 / fps_elapsed.as_secs_f32();
            self.frame_count = 0;
            self.last_fps_time = current_time;
        }
    }
  
    pub fn get_delta_time(&self) -> f32 {
        self.delta_time
    }

    pub fn get_fps(&self) -> f32 {
        self.fps
    }

}