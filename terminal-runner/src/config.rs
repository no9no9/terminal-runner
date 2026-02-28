use std::error::Error;
use std::fs;
use std::path::Path;

use crossterm::style::Color;
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

/// Decoration settings separate from gameplay difficulty settings.
/// Controls the visual appearance of characters and colors.
#[derive(Debug, Deserialize, Clone)]
pub struct DecorationConfig {
    pub player_char: String,
    pub obstacle_char: String,
    pub ground_char: String,
    pub player_color: String,
    pub obstacle_color: String,
    pub ground_color: String,
    pub background_color: String,
}

impl DecorationConfig {
    pub fn load(path: impl AsRef<Path>) -> Result<Self, Box<dyn Error>> {
        let text = fs::read_to_string(path)?;
        let config: DecorationConfig = serde_json::from_str(&text)?;
        config.validate()?;
        Ok(config)
    }

    /// Parse all color strings and return a `ParsedDecoration` for efficient use at runtime.
    pub fn parse(&self) -> Result<ParsedDecoration, Box<dyn Error>> {
        Ok(ParsedDecoration {
            player_char: self.player_char.clone(),
            obstacle_char: self.obstacle_char.clone(),
            ground_char: self.ground_char.clone(),
            player_color: parse_color(&self.player_color)?,
            obstacle_color: parse_color(&self.obstacle_color)?,
            ground_color: parse_color(&self.ground_color)?,
            background_color: parse_color(&self.background_color)?,
        })
    }

    fn validate(&self) -> Result<(), Box<dyn Error>> {
        if self.player_char.chars().count() != 1 {
            return Err("decoration config error: player_char must be exactly one character".into());
        }
        if self.obstacle_char.chars().count() != 1 {
            return Err("decoration config error: obstacle_char must be exactly one character".into());
        }
        if self.ground_char.chars().count() != 1 {
            return Err("decoration config error: ground_char must be exactly one character".into());
        }
        // Validate color strings by attempting to parse them
        parse_color(&self.player_color)?;
        parse_color(&self.obstacle_color)?;
        parse_color(&self.ground_color)?;
        parse_color(&self.background_color)?;
        Ok(())
    }
}

/// Pre-parsed decoration values for efficient use during the render loop.
#[derive(Debug, Clone)]
pub struct ParsedDecoration {
    pub player_char: String,
    pub obstacle_char: String,
    pub ground_char: String,
    pub player_color: Color,
    pub obstacle_color: Color,
    pub ground_color: Color,
    pub background_color: Color,
}

/// Parses a color name string into a crossterm `Color`.
/// Supported names: black, dark_grey, red, dark_red, green, dark_green,
/// yellow, dark_yellow, blue, dark_blue, magenta, dark_magenta, cyan,
/// dark_cyan, white, grey.
pub fn parse_color(name: &str) -> Result<Color, Box<dyn Error>> {
    match name.to_lowercase().as_str() {
        "black" => Ok(Color::Black),
        "dark_grey" | "dark_gray" => Ok(Color::DarkGrey),
        "red" => Ok(Color::Red),
        "dark_red" => Ok(Color::DarkRed),
        "green" => Ok(Color::Green),
        "dark_green" => Ok(Color::DarkGreen),
        "yellow" => Ok(Color::Yellow),
        "dark_yellow" => Ok(Color::DarkYellow),
        "blue" => Ok(Color::Blue),
        "dark_blue" => Ok(Color::DarkBlue),
        "magenta" => Ok(Color::Magenta),
        "dark_magenta" => Ok(Color::DarkMagenta),
        "cyan" => Ok(Color::Cyan),
        "dark_cyan" => Ok(Color::DarkCyan),
        "white" => Ok(Color::White),
        "grey" | "gray" => Ok(Color::Grey),
        _ => Err(format!("unknown color: '{name}'").into()),
    }
}