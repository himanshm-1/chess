use macroquad::{prelude::*, ui::root_ui};
use miniquad::conf::Icon;
use std::collections::HashMap;

pub fn other_things(board_white: &Texture2D) -> ((f32, f32), [(f32, f32); 64], Mode) {
    (
        board_pos(board_white),
        set_square_position(
            &mut [(0., 0.); 64],
            board_pos(board_white).0,
            board_pos(board_white).1,
        ),
        Mode::Menu,
    )
}

pub fn all_hashmaps(
    _bitboards: HashMap<&'static str, u64>,
    _piece_index: HashMap<&'static str, i32>,
) -> (HashMap<&'static str, u64>, HashMap<&'static str, i32>) {
    (set_bitboard(_bitboards), set_piece_index(_piece_index))
}

pub async fn chess_piece() -> Texture2D {
    load_texture("chess_pieces_array.png").await.unwrap()
}

pub async fn white_board() -> Texture2D {
    load_texture("chess_white_perspective_board.png")
        .await
        .unwrap()
}

pub async fn black_board() -> Texture2D {
    load_texture("chess_black_perspective_board.png")
        .await
        .unwrap()
}

pub async fn all_textures() -> (Texture2D, Texture2D, Texture2D) {
    (
        chess_piece().await,
        white_board().await,
        black_board().await,
    )
}

pub fn window_conf() -> Conf {
    Conf {
        window_title: "Chess".to_owned(),
        window_resizable: false,
        icon: Some(set_icon()),
        ..Default::default()
    }
}

pub fn ui(mut mode: Mode) -> Mode {
    root_ui().label(label_pos(), "Choose which color you want to play as.");
    if root_ui().button(black_pos(), "Black") {
        mode = Mode::Black;
    } else if root_ui().button(white_pos(), "White") {
        mode = Mode::White;
    }
    mode
}

pub fn q_width() -> f32 {
    measure_text("Choose which color you want to play as.", None, 15, 1.0).width / 2.
}

pub fn height() -> f32 {
    (window_conf().window_height / 2 - 40) as f32
}

pub fn set_piece_index(mut _piece_index: HashMap<&str, i32>) -> HashMap<&str, i32> {
    _piece_index.insert("BRook", 3);
    _piece_index.insert("BKnight", 4);
    _piece_index.insert("BBishop", 5);
    _piece_index.insert("BKing", 2);
    _piece_index.insert("BQueen", 1);
    _piece_index.insert("BPawn", 6);
    _piece_index.insert("WRook", 9);
    _piece_index.insert("WKnight", 10);
    _piece_index.insert("WBishop", 11);
    _piece_index.insert("WQueen", 7);
    _piece_index.insert("WKing", 8);
    _piece_index.insert("WPawn", 12);
    _piece_index
}

pub fn board_pos(board_white: &Texture2D) -> (f32, f32) {
    (
        (window_conf().window_width / 2) as f32 - board_white.width() / 2.,
        (window_conf().window_height / 2) as f32 - board_white.height() / 2.,
    )
}

pub fn black_pos() -> Option<Vec2> {
    Some(Vec2::new((width() - 60) as f32, height() + 20.))
}
pub fn white_pos() -> Option<Vec2> {
    Some(Vec2::new((width() + 20) as f32, height() + 20.))
}

#[derive(PartialEq, Eq)]
pub enum Mode {
    Menu,
    White,
    Black,
}

pub fn set_square_position(
    square_position: &mut [(f32, f32); 64],
    offset_x: f32,
    offset_y: f32,
) -> [(f32, f32); 64] {
    let mut current = 0;
    for j in 0..8 {
        for i in 0..8 {
            square_position[current] = (offset_x + i as f32 * 32., offset_y + j as f32 * 32.);
            current += 1;
        }
    }
    *square_position
}

pub const fn set_icon() -> Icon {
    const LOGO16: &[u8; 1024] = include_bytes!("logo16.rgba");
    const LOGO32: &[u8; 4096] = include_bytes!("logo32.rgba");
    const LOGO64: &[u8; 16384] = include_bytes!("logo64.rgba");
    Icon {
        small: *LOGO16,
        medium: *LOGO32,
        big: *LOGO64,
    }
}

pub fn set_bitboard(mut _bitboards: HashMap<&str, u64>) -> HashMap<&str, u64> {
    _bitboards.insert(
        "BRook",
        0b10000001_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
    );
    _bitboards.insert(
        "BKnight",
        0b01000010_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
    );
    _bitboards.insert(
        "BBishop",
        0b00100100_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
    );
    _bitboards.insert(
        "BKing",
        0b00001000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
    );
    _bitboards.insert(
        "BQueen",
        0b00010000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
    );
    _bitboards.insert(
        "BPawn",
        0b00000000_11111111_00000000_00000000_00000000_00000000_00000000_00000000,
    );
    _bitboards.insert(
        "WRook",
        0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_10000001,
    );
    _bitboards.insert(
        "WKnight",
        0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_01000010,
    );
    _bitboards.insert(
        "WBishop",
        0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00100100,
    );
    _bitboards.insert(
        "WQueen",
        0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00010000,
    );
    _bitboards.insert(
        "WKing",
        0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00001000,
    );
    _bitboards.insert(
        "WPawn",
        0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_00000000,
    );
    _bitboards.insert(
        "BTotal",
        0b11111111_11111111_00000000_00000000_00000000_00000000_00000000_00000000,
    );
    _bitboards.insert(
        "WTotal",
        0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_11111111,
    );
    _bitboards.insert(
        "Total",
        0b11111111_11111111_00000000_00000000_00000000_00000000_11111111_11111111,
    );

    _bitboards
}

pub fn width() -> i32 {
    window_conf().window_width / 2
}

pub fn label_pos() -> Option<Vec2> {
    Some(Vec2::new(width() as f32 - q_width(), height()))
}
