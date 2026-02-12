use macroquad::prelude::*;
use macroquad::ui::root_ui;
use miniquad::conf::Icon;

#[derive(PartialEq, Eq)]
enum Mode {
    Menu,
    White,
    Black,
}

#[macroquad::main(window_conf)]
async fn main() {
    let _square_position = set_square_position(&mut [(0, 0); 64], 0, 0);

    let mut _bitboards: [u64; 15] = [
        0b10000001_00000000_00000000_00000000_00000000_00000000_00000000_00000000, // BRook:      0
        0b01000010_00000000_00000000_00000000_00000000_00000000_00000000_00000000, // BKnight:    1
        0b00100100_00000000_00000000_00000000_00000000_00000000_00000000_00000000, // BBishop:    2
        0b00001000_00000000_00000000_00000000_00000000_00000000_00000000_00000000, // BKing:      3
        0b00010000_00000000_00000000_00000000_00000000_00000000_00000000_00000000, // BQueen:     4
        0b00000000_11111111_00000000_00000000_00000000_00000000_00000000_00000000, // BPawn:      5
        0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_10000001, // WRook:      6
        0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_01000010, // WKnight:    7
        0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00100100, // WBishop:    8
        0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00010000, // WQueen:     9
        0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00001000, // WKing:     10
        0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_00000000, // WPawn:     11
        0b11111111_11111111_00000000_00000000_00000000_00000000_00000000_00000000, // BOccupied: 12
        0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_11111111, // WOccupied: 13
        0b11111111_11111111_00000000_00000000_00000000_00000000_11111111_11111111, // TOccupied: 14
    ];

    let board_white: Texture2D = load_texture("chess_white_perspective_board.png")
        .await
        .unwrap();

    let board_black: Texture2D = load_texture("chess_black_perspective_board.png")
        .await
        .unwrap();

    let mut mode = Mode::Menu;

    next_frame().await;

    let board_pos: (f32, f32) = (
        (window_conf().window_width / 2) as f32 - board_white.width() / 2.,
        (window_conf().window_height / 2) as f32 - board_white.height() / 2.,
    );

    loop {
        clear_background(GOLD);

        let height = (window_conf().window_height / 2 - 40) as f32;



        if mode == Mode::Menu {
            root_ui().label(
                Some(Vec2::new(
                    (window_conf().window_width / 2) as f32
                        - measure_text("Choose which color you want to play as.", None, 15, 1.0)
                            .width/2.,
                    height,
                )),
                "Choose which color you want to play as.",
            );
            if root_ui().button(
                Some(Vec2::new(
                    (window_conf().window_width / 2 - 60) as f32,
                    height + 20.,
                )),
                "Black",
            ) {
                mode = Mode::Black;
            } else if root_ui().button(
                Some(Vec2::new(
                    (window_conf().window_width / 2 + 20) as f32,
                    height + 20.,
                )),
                "White",
            ) {
                mode = Mode::White;
            }
        } else if mode == Mode::Black {
            draw_texture(&board_black, board_pos.0, board_pos.1, WHITE);
        } else if mode == Mode::White {
            draw_texture(&board_white, board_pos.0, board_pos.1, WHITE);
        }

        next_frame().await;
    }
}

fn set_square_position(
    square_position: &mut [(isize, isize); 64],
    offset_x: isize,
    offset_y: isize,
) -> [(isize, isize); 64] {
    let mut current = 0;
    for j in 0..8 {
        for i in 0..8 {
            square_position[current] = (offset_x + i * 32, offset_y + j * 32);
            current += 1;
        }
    }
    return *square_position;
}

fn set_icon() -> Icon {
    const LOGO16: &[u8; 1024] = include_bytes!("logo16.rgba");
    const LOGO32: &[u8; 4096] = include_bytes!("logo32.rgba");
    const LOGO64: &[u8; 16384] = include_bytes!("logo64.rgba");
    Icon {
        small: *LOGO16,
        medium: *LOGO32,
        big: *LOGO64,
    }
}
fn window_conf() -> Conf {
    Conf {
        window_title: "Chess".to_owned(),
        window_resizable: false,
        icon: Some(set_icon()),
        ..Default::default()
    }
}
