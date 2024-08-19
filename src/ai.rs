// src/ai.rs
#![allow(warnings)]

use rand::random;

pub fn ai_random_move(field: &mut Vec<i8>) {
    let mut empty_cells: Vec<i8> = Vec::new();

    empty_cells = field
        .iter()
        .enumerate()
        .filter(|(_, &cell)| cell == 0)
        .map(|(index, _)| index as i8)
        .collect();
    if empty_cells.len() == 0 {
        return;
    }
    let random_index = rand::random::<usize>() % empty_cells.len();
    field[empty_cells[random_index] as usize] = -1;
}
