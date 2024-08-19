// src/draw.rs
#![allow(warnings)]

use macroquad::prelude::*;

pub fn draw_grid() {
    draw_line(
        0.0,
        screen_height() * 0.3,
        screen_width(),
        screen_height() * 0.3,
        10.0,
        BLACK,
    );

    draw_line(
        0.0,
        screen_height() * 0.66,
        screen_width(),
        screen_height() * 0.66,
        10.0,
        BLACK,
    );

    draw_line(
        screen_width() * 0.33,
        0.0,
        screen_width() * 0.33,
        screen_height(),
        10.0,
        BLACK,
    );

    draw_line(
        screen_width() * 0.66,
        0.0,
        screen_width() * 0.66,
        screen_height(),
        10.0,
        BLACK,
    );
}

pub fn is_in_grid(pos: (f32, f32)) -> i32 {
    match pos {
        (x, y) if x < screen_width() * 0.33 && y < screen_height() * 0.3 => 0,
        (x, y)
            if x > screen_width() * 0.33
                && x < screen_width() * 0.66
                && y < screen_height() * 0.3 =>
        {
            1
        }
        (x, y) if x > screen_width() * 0.66 && y < screen_height() * 0.3 => 2,
        (x, y)
            if x < screen_width() * 0.33
                && y > screen_height() * 0.3
                && y < screen_height() * 0.66 =>
        {
            3
        }
        (x, y)
            if x > screen_width() * 0.33
                && x < screen_width() * 0.66
                && y > screen_height() * 0.3
                && y < screen_height() * 0.66 =>
        {
            4
        }
        (x, y)
            if x > screen_width() * 0.66
                && y > screen_height() * 0.3
                && y < screen_height() * 0.66 =>
        {
            5
        }
        (x, y) if x < screen_width() * 0.33 && y > screen_height() * 0.66 => 6,
        (x, y)
            if x > screen_width() * 0.33
                && x < screen_width() * 0.66
                && y > screen_height() * 0.66 =>
        {
            7
        }
        (x, y) if x > screen_width() * 0.66 && y > screen_height() * 0.66 => 8,
        _ => -1,
    }
}

pub fn draw_x_and_o(field: &Vec<i8>) {
    for (index, value) in field.iter().enumerate() {
        let x = index % 3;
        let y = index / 3;

        match value {
            1 => {
                draw_line(
                    screen_width() * 0.0 + (screen_width() * 0.33 * x as f32),
                    screen_height() * 0.0 + (screen_height() * 0.33 * y as f32),
                    screen_width() * 0.0 + (screen_width() * 0.33 * (x + 1) as f32),
                    screen_height() * 0.0 + (screen_height() * 0.33 * (y + 1) as f32),
                    10.0,
                    BLUE,
                );

                draw_line(
                    screen_width() * 0.0 + (screen_width() * 0.33 * (x + 1) as f32),
                    screen_height() * 0.0 + (screen_height() * 0.33 * y as f32),
                    screen_width() * 0.0 + (screen_width() * 0.33 * x as f32),
                    screen_height() * 0.0 + (screen_height() * 0.33 * (y + 1) as f32),
                    10.0,
                    BLUE,
                );
            }
            -1 => {
                draw_circle_lines(
                    screen_width() * 0.16 + (screen_width() * 0.33 * x as f32),
                    screen_height() * 0.16 + (screen_height() * 0.33 * y as f32),
                    80.0,
                    10.0,
                    RED,
                );
            }
            _ => {}
        }
    }
}
