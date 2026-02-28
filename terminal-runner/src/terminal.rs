use std::error::Error;
use std::io::Stdout;
use std::time::Duration;

use crossterm::cursor::{Hide, Show};
use crossterm::event::{self, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen,
};

pub enum GameOverAction {
    Restart,
    Quit,
}

pub fn setup_terminal(out: &mut Stdout) -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    execute!(out, EnterAlternateScreen, Hide, Clear(ClearType::All))?;
    Ok(())
}

pub fn restore_terminal(out: &mut Stdout) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    execute!(out, Show, LeaveAlternateScreen)?;
    Ok(())
}

pub fn wait_for_game_over_action() -> Result<GameOverAction, Box<dyn Error>> {
    loop {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('r') => return Ok(GameOverAction::Restart),
                    KeyCode::Esc | KeyCode::Char('q') => return Ok(GameOverAction::Quit),
                    _ => {}
                }
            }
        }
    }
}