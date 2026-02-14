/*
 * I’m not sure why this happens, but you can’t pass a function from another module directly into macroquad::main().
 *
 * If you want to use custom pieces, make sure each one in the new array is exactly 64 × 64 pixels.
 * If they are not, update the constant SINGLE_SQUARE_DIMENSION in utils.rs.
 * The board dimensions must be adjusted accordingly, because changing SINGLE_SQUARE_DIMENSION also affects
 * _square_position, which determines where pieces are placed on the board. All squares on the board use the same
 * dimensions (SINGLE_SQUARE_DIMENSION × SINGLE_SQUARE_DIMENSION), so changing this value will affect rendering.
 * You may also need to adjust the board layout for different perspectives after making this change.
 *
 * This file defines the top-level layer; most of the implementation is contained in utils.rs.
 */

mod utils;
use macroquad::prelude::*;
use std::collections::HashMap;

#[macroquad::main(conf)]
async fn main() {
    let (mut _bitboards, piece_index) = utils::all_hashmaps(HashMap::new(), HashMap::new());
    let (piece_array, board_white, board_black) = utils::all_textures().await;
    let (board_pos, mut square_position, mut mode) = utils::other_things(&board_white);

    let mut reverse_for_white = true;

    loop {
        clear_background(GOLD);

        match mode {
            utils::Mode::Menu => mode = utils::ui(mode),
            utils::Mode::White => {
                draw_texture(&board_white, board_pos.0, board_pos.1, WHITE);
                if reverse_for_white {
                    square_position.reverse();
                    reverse_for_white = false;
                }
                for (key, _value) in &_bitboards {
                    utils::draw_piece(key, 23, &square_position, &piece_array, &piece_index);
                }

            }
            utils::Mode::Black => {
                draw_texture(&board_black, board_pos.0, board_pos.1, WHITE);
            }
        }

        next_frame().await;
    }
}

fn conf() -> Conf {
    utils::window_conf()
}
