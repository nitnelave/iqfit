use crate::board::Board;
use crate::pieces::*;
use std::collections::HashSet;

const COLOR_LIST: [Color; 10] = [
    Color::Yellow,
    Color::Orange,
    Color::Red,
    Color::Pink,
    Color::LightGreen,
    Color::Green,
    Color::LightBlue,
    Color::Blue,
    Color::DeepBlue,
    Color::Purple,
];

const FACE_LIST: [Face; 2] = [Face::A, Face::B];

const ORIENTATION_LIST: [Orientation; 4] = [
    Orientation::Up,
    Orientation::Right,
    Orientation::Down,
    Orientation::Left,
];

fn get_colorset() -> HashSet<Color> {
    let mut pieces_left = HashSet::<Color>::with_capacity(10);
    for c in COLOR_LIST.iter() {
        pieces_left.insert(*c);
    }
    pieces_left
}

fn solve_rec<B: Board>(board: &mut B, colors_left: &mut HashSet<Color>) -> bool {
    let index = board.first_empty_cell();
    if index.is_none() {
        return true;
    }
    let index = index.unwrap();
    let mut piece = PlacedPiece {
        piece: Piece::new(),
        top_left: index,
    };
    for c in COLOR_LIST.iter() {
        if !colors_left.remove(c) {
            continue;
        }
        piece.piece.set_color(*c);
        for face in FACE_LIST.iter() {
            piece.piece.set_face(*face);
            for orientation in ORIENTATION_LIST.iter() {
                piece.piece.set_orientation(*orientation);
                if board.place_piece(piece) {
                    if solve_rec(board, colors_left) {
                        return true;
                    }
                    board.pop_piece();
                }
            }
        }
        colors_left.insert(*c);
    }
    false
}

pub fn solve<B: Board>(mut board: B) -> Option<B> {
    let mut colors_left = get_colorset();
    for p in board.piece_list() {
        assert!(colors_left.remove(&p.piece.color()));
    }

    if solve_rec(&mut board, &mut colors_left) {
        Some(board)
    } else {
        None
    }
}
