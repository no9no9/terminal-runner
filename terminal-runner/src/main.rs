use std::error::Error;
use std::io::stdout;

mod config;
mod game;
mod render;
mod terminal;

use config::{AppConfig, DecorationConfig};
use game::run_game;
use terminal::{restore_terminal, setup_terminal};

fn main() -> Result<(), Box<dyn Error>> {
    let config = AppConfig::load("config/game.json")?;
    let decoration = DecorationConfig::load("config/decoration.json")?.parse()?;

    let mut out = stdout();
    setup_terminal(&mut out)?;

    let run_result = run_game(&mut out, &config, &decoration);
    restore_terminal(&mut out)?;

    run_result
}