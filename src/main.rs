#![allow(warnings)]

use macroquad::prelude::*;

mod ai;
mod draw;
mod game;

enum GameState {
    Menu,
    Playing,
    PlayerWins,
    AIWins,
}

#[macroquad::main("TicTacToe")]
async fn main() {
    let mut field: Vec<i8> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut winner = 0;
    let mut state = GameState::Menu;
    loop {
        match state {
            GameState::Menu => {
                state = game::menu();
            }
            GameState::Playing => {
                state = game::play_game(&mut field, &mut winner);
            }
            GameState::PlayerWins => {
                draw_text(
                    "YOU WON",
                    screen_width() * 0.5 - screen_width() * 0.25,
                    screen_height() * 0.5,
                    100.0,
                    BLUE,
                );
            }
            GameState::AIWins => {
                draw_text(
                    "YOU LOST",
                    screen_width() * 0.5 - screen_width() * 0.25,
                    screen_height() * 0.5,
                    100.0,
                    RED,
                );
            }
        }

        next_frame().await
    }
}
