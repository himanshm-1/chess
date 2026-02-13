mod utils;
use macroquad::{prelude::*};
use std::collections::HashMap;

#[macroquad::main(conf)]
async fn main() {
    let mut _bitboards: HashMap<&str, u64> = HashMap::new();
    _bitboards = utils::set_bitboard(_bitboards); // Naming Scheme: BRook, WKnight, BTotal, WTotal, Total etc.

    let board_white: Texture2D = utils::white_board().await;
    let board_black: Texture2D = utils::black_board().await;
    let _piece_array: Texture2D = load_texture("chess_pieces_array.png").await.unwrap();

    next_frame().await; // To initialize the window so, window_height and width are not zero.
    let board_pos = utils::board_pos(&board_white);
    let _square_position = utils::set_square_position(&mut [(0., 0.); 64], board_pos.0, board_pos.1);
    let mut _piece_index: HashMap<&str, i32> = HashMap::new();
    _piece_index = utils::set_piece_index(_piece_index);

    let mut mode = utils::Mode::Menu;

    loop {
        clear_background(GOLD);

        if mode == utils::Mode::Menu {
            mode = utils::ui(mode);
        } else if mode == utils::Mode::Black {
            draw_texture(&board_black, board_pos.0, board_pos.1, WHITE);
        } else if mode == utils::Mode::White {
            draw_texture(&board_white, board_pos.0, board_pos.1, WHITE);
        }

        next_frame().await;
    }
}


/* I don't know why this happens but I can't put a function from another module directly into macroquad::main() */
fn conf() -> Conf {
    utils::window_conf()
}


