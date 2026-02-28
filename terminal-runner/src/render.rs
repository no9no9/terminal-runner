use std::error::Error;
use std::io::{Stdout, Write};

use crossterm::cursor::MoveTo;
use crossterm::queue;
use crossterm::style::{Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType};

use crate::config::ParsedDecoration;
use crate::game::Obstacle;

pub fn draw_frame(
    out: &mut Stdout,
    width: u16,
    height: u16,
    ground_y: u16,
    obstacle_width: u16,
    player_x: u16,
    player_y: u16,
    obstacles: &[Obstacle],
    score: u64,
    decoration: &ParsedDecoration,
) -> Result<(), Box<dyn Error>> {
    queue!(out, SetBackgroundColor(decoration.background_color), MoveTo(0, 0), Clear(ClearType::All))?;

    let hud = "terminal-runner  [←/→ or A/D] Move  [Space/↑/W] Double Jump  [Q/Esc] Quit";
    queue!(out, ResetColor, SetBackgroundColor(decoration.background_color), MoveTo(0, 0), Print(hud))?;
    queue!(out, MoveTo(0, 1), Print(format!("Score: {score}")))?;

    queue!(out, SetForegroundColor(decoration.ground_color))?;
    for x in 0..width {
        queue!(out, MoveTo(x, ground_y), Print(&decoration.ground_char))?;
    }

    queue!(out, SetForegroundColor(decoration.obstacle_color))?;
    for obstacle in obstacles {
        for w_offset in 0..obstacle_width {
            let x = obstacle.x + w_offset as i16;
            if x < 0 || x >= width as i16 {
                continue;
            }
            for y in 0..obstacle.height {
                let yy = ground_y.saturating_sub(y);
                queue!(out, MoveTo(x as u16, yy), Print(&decoration.obstacle_char))?;
            }
        }
    }

    if player_y < height && player_x < width {
        queue!(
            out,
            SetForegroundColor(decoration.player_color),
            MoveTo(player_x, player_y),
            Print(&decoration.player_char)
        )?;
    }

    queue!(out, ResetColor)?;
    out.flush()?;
    Ok(())
}

pub fn draw_game_over(
    out: &mut Stdout,
    width: u16,
    height: u16,
    score: u64,
) -> Result<(), Box<dyn Error>> {
    let msg = format!("GAME OVER - Score: {score}  (R: Restart / Q or Esc: Quit)");
    let x = (width.saturating_sub(msg.len() as u16)) / 2;
    let y = height / 2;
    queue!(out, MoveTo(x, y), Print(msg))?;
    out.flush()?;
    Ok(())
}