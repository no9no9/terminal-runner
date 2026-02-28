use std::error::Error;
use std::io::stdout;

mod config;
mod game;
mod render;
mod terminal;

use config::AppConfig;
use game::run_game;
use terminal::{restore_terminal, setup_terminal};

fn main() -> Result<(), Box<dyn Error>> {
    let config = AppConfig::load("config/game.json")?;

    let mut out = stdout();
    setup_terminal(&mut out)?;

    let run_result = run_game(&mut out, &config);
    restore_terminal(&mut out)?;

    run_result
}