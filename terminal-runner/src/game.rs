use std::error::Error;
use std::io::Stdout;
use std::time::Instant;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::terminal::size;
use rand::Rng;

use crate::config::AppConfig;
use crate::render::{draw_frame, draw_game_over};
use crate::terminal::{wait_for_game_over_action, GameOverAction};

#[derive(Clone, Copy)]
pub struct Obstacle {
    pub x: i16,
    pub height: u16,
    pub passed: bool,
}

pub fn run_game(out: &mut Stdout, config: &AppConfig) -> Result<(), Box<dyn Error>> {
    let frame_time = std::time::Duration::from_millis(config.frame_time_ms);

    let mut rng = rand::thread_rng();

    'round: loop {
        let (mut width, mut height) = normalized_size(size()?, config);
        let mut ground_y = height.saturating_sub(2);

        let mut player_x = config.player_start_x.min(width.saturating_sub(1));
        let mut player_y = ground_y as f32 - 1.0;
        let mut player_vy = 0.0_f32;
        let mut jumps_remaining: u8 = config.max_jumps;

        let mut obstacles: Vec<Obstacle> = Vec::new();
        let mut frame_count: u64 = 0;
        let mut score: u64 = 0;
        let mut spawn_in = rng.gen_range(config.spawn_interval_min..config.spawn_interval_max);

        loop {
            let frame_start = Instant::now();

            while event::poll(Duration::from_millis(0))? {
                match event::read()? {
                    Event::Key(key)
                        if key.kind == KeyEventKind::Press
                            || key.kind == KeyEventKind::Repeat =>
                    {
                        match key.code {
                            KeyCode::Esc | KeyCode::Char('q') => {
                                return Ok(());
                            }
                            KeyCode::Char(' ') | KeyCode::Up | KeyCode::Char('w') => {
                                if key.kind == KeyEventKind::Press && jumps_remaining > 0 {
                                    player_vy = config.jump_velocity;
                                    jumps_remaining -= 1;
                                }
                            }
                            KeyCode::Left | KeyCode::Char('a') => {
                                player_x = player_x.saturating_sub(1);
                            }
                            KeyCode::Right | KeyCode::Char('d') => {
                                if player_x + 1 < width {
                                    player_x += 1;
                                }
                            }
                            _ => {}
                        }
                    }
                    Event::Resize(new_w, new_h) => {
                        let (nw, nh) = normalized_size((new_w, new_h), config);
                        width = nw;
                        height = nh;
                        ground_y = height.saturating_sub(2);
                        if player_y > ground_y as f32 - 1.0 {
                            player_y = ground_y as f32 - 1.0;
                            player_vy = 0.0;
                            jumps_remaining = config.max_jumps;
                        }
                        if player_x >= width {
                            player_x = width.saturating_sub(1);
                        }
                    }
                    _ => {}
                }
            }

            player_y += player_vy;
            player_vy += config.gravity;
            let ground_player_y = ground_y as f32 - 1.0;
            if player_y >= ground_player_y {
                player_y = ground_player_y;
                player_vy = 0.0;
                jumps_remaining = config.max_jumps;
            }

            spawn_in = spawn_in.saturating_sub(1);
            if spawn_in == 0 {
                let h = rng.gen_range(config.obstacle_height_min..=config.obstacle_height_max);
                obstacles.push(Obstacle {
                    x: width as i16 - 1,
                    height: h,
                    passed: false,
                });
                spawn_in = rng.gen_range(config.spawn_interval_min..config.spawn_interval_max);
            }

            frame_count += 1;
            if frame_count % config.obstacle_move_interval_frames == 0 {
                for obstacle in &mut obstacles {
                    obstacle.x -= 1;
                }
            }

            let player_y_u16 = player_y.round().max(0.0) as u16;
            for obstacle in &mut obstacles {
                if !obstacle.passed && obstacle.x + config.obstacle_width as i16 <= player_x as i16 {
                    obstacle.passed = true;
                    score += 1;
                }
            }
            obstacles.retain(|obs| obs.x + config.obstacle_width as i16 > 0);

            let hit = check_collision(
                player_x,
                player_y_u16,
                ground_y,
                config.obstacle_width,
                &obstacles,
            );
            draw_frame(
                out,
                width,
                height,
                ground_y,
                config.obstacle_width,
                player_x,
                player_y_u16,
                &obstacles,
                score,
            )?;

            if hit {
                draw_game_over(out, width, height, score)?;
                match wait_for_game_over_action()? {
                    GameOverAction::Restart => continue 'round,
                    GameOverAction::Quit => return Ok(()),
                }
            }

            let elapsed = frame_start.elapsed();
            if elapsed < frame_time {
                std::thread::sleep(frame_time - elapsed);
            }
        }
    }
}

fn normalized_size((w, h): (u16, u16), config: &AppConfig) -> (u16, u16) {
    (w.max(config.min_screen_width), h.max(config.min_screen_height))
}

fn check_collision(
    player_x: u16,
    player_y: u16,
    ground_y: u16,
    obstacle_width: u16,
    obstacles: &[Obstacle],
) -> bool {
    for obstacle in obstacles {
        let left = obstacle.x;
        let right = obstacle.x + obstacle_width as i16 - 1;
        let player_x = player_x as i16;

        if player_x >= left && player_x <= right {
            let obstacle_top = ground_y.saturating_sub(obstacle.height.saturating_sub(1));
            if player_y >= obstacle_top {
                return true;
            }
        }
    }
    false
}