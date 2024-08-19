// src/game.rs
#![allow(warnings)]

use crate::ai;
use crate::draw;
use crate::GameState;
use macroquad::prelude::*;

pub fn play_game(mut field: &mut Vec<i8>, winner: &mut i8) -> GameState {
    clear_background(GRAY);

    draw::draw_grid();
    draw::draw_x_and_o(&field);
    draw_circle(mouse_position().0, mouse_position().1, 5.0, YELLOW);

    // draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

    if is_mouse_button_pressed(MouseButton::Left) {
        // println!("Grid Number: {:?}", draw::is_in_grid(mouse_position()));
        if field[draw::is_in_grid(mouse_position()) as usize] == 0 {
            if *winner == 0 {
                field[draw::is_in_grid(mouse_position()) as usize] = 1;
                *winner = check_winner(&field);
            }
            if *winner == 0 {
                ai::ai_random_move(&mut field);
                *winner = check_winner(&field);
            }
            // println!("Game Statuts: {:?}", winner);
        }
    }
    // if *winner != 0 {
    //     if *winner == 1 {
    //         draw_text("Player Wins!", 100.0, 100.0, 60.0, BLACK);
    //     } else {
    //         draw_text("AI Wins!", 100.0, 100.0, 60.0, BLACK);
    //     }
    // }
    match *winner {
        1 => GameState::PlayerWins,
        -1 => GameState::AIWins,
        _ => GameState::Playing,
    }
}

pub fn check_winner(field: &Vec<i8>) -> i8 {
    let winning_positions: [[usize; 3]; 8] = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6],
    ];

    for position in winning_positions.iter() {
        if field[position[0]] == 1 && field[position[1]] == 1 && field[position[2]] == 1 {
            return 1;
        } else if field[position[0]] == -1 && field[position[1]] == -1 && field[position[2]] == -1 {
            return -1;
        }
    }

    return 0;
}

pub fn menu() -> GameState {
    clear_background(GRAY);
    let screen_width = screen_width();
    let screen_height = screen_height();
    let text_width = measure_text("Tic Tac Toe", None, 30, 1.0).width;
    let text_height = measure_text("Tic Tac Toe", None, 30, 1.0).height;
    let text_x = (screen_width - text_width) / 2.0;
    let text_y = (screen_height - text_height) / 2.0;

    draw_text("Tic Tac Toe", text_x, text_y, 30.0, DARKGRAY);
    draw_text("Press Enter to Play", text_x, text_y + 40.0, 20.0, DARKGRAY);

    if is_key_pressed(KeyCode::Enter) {
        GameState::Playing
    } else {
        GameState::Menu
    }
}
