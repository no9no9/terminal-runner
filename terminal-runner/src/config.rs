use std::error::Error;
use std::fs;
use std::path::Path;

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub frame_time_ms: u64,
    pub gravity: f32,
    pub jump_velocity: f32,
    pub max_jumps: u8,
    pub player_start_x: u16,
    pub obstacle_width: u16,
    pub obstacle_height_min: u16,
    pub obstacle_height_max: u16,
    pub spawn_interval_min: u16,
    pub spawn_interval_max: u16,
    pub obstacle_move_interval_frames: u64,
    pub min_screen_width: u16,
    pub min_screen_height: u16,
}

impl AppConfig {
    pub fn load(path: impl AsRef<Path>) -> Result<Self, Box<dyn Error>> {
        let text = fs::read_to_string(path)?;
        let config: AppConfig = serde_json::from_str(&text)?;
        config.validate()?;
        Ok(config)
    }

    fn validate(&self) -> Result<(), Box<dyn Error>> {
        if self.max_jumps == 0 {
            return Err("config error: max_jumps must be >= 1".into());
        }
        if self.obstacle_width == 0 {
            return Err("config error: obstacle_width must be >= 1".into());
        }
        if self.obstacle_move_interval_frames == 0 {
            return Err("config error: obstacle_move_interval_frames must be >= 1".into());
        }
        if self.obstacle_height_min == 0 || self.obstacle_height_min > self.obstacle_height_max {
            return Err("config error: obstacle_height_min must be <= obstacle_height_max and >= 1".into());
        }
        if self.spawn_interval_min == 0 || self.spawn_interval_min >= self.spawn_interval_max {
            return Err("config error: spawn_interval_min must be < spawn_interval_max and >= 1".into());
        }
        Ok(())
    }
}