mod utils;
use macroquad::prelude::*;
use std::collections::HashMap;

#[macroquad::main(conf)]
async fn main() {

    let (_bitboards, _piece_index) = utils::all_hashmaps(HashMap::new(), HashMap::new());
    let (_piece_array, board_white, board_black) = utils::all_textures().await;
    let (board_pos, _square_position, mut mode) = utils::other_things(&board_white);

    loop {
        clear_background(GOLD);

        match mode {
            utils::Mode::Menu => mode = utils::ui(mode),
            utils::Mode::White => {
                draw_texture(&board_black, board_pos.0, board_pos.1, WHITE);

            },
            utils::Mode::Black => {
                draw_texture(&board_white, board_pos.0, board_pos.1, WHITE);

            },
        }

        next_frame().await;
    }


}

/* I don't know why this happens but I can't put a function from another module directly into macroquad::main() */
fn conf() -> Conf {
    utils::window_conf()
}
