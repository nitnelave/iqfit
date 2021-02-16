use crate::pieces::*;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref PIECES_49: [PlacedPiece; 3] = [
        PlacedPiece {
            piece: Piece::new()
                .with_color(Color::Yellow)
                .with_face(Face::B)
                .with_orientation(Orientation::Up),
            top_left: 10,
        },
        PlacedPiece {
            piece: Piece::new()
                .with_color(Color::Blue)
                .with_face(Face::A)
                .with_orientation(Orientation::Up),
            top_left: 14,
        },
        PlacedPiece {
            piece: Piece::new()
                .with_color(Color::Green)
                .with_face(Face::A)
                .with_orientation(Orientation::Up),
            top_left: 18,
        },
    ];
    pub static ref PIECES_117: [PlacedPiece; 2] = [
        PlacedPiece {
            piece: Piece::new()
                .with_color(Color::LightBlue)
                .with_face(Face::B)
                .with_orientation(Orientation::Right),
            top_left: 3,
        },
        PlacedPiece {
            piece: Piece::new()
                .with_color(Color::DeepBlue)
                .with_face(Face::A)
                .with_orientation(Orientation::Left),
            top_left: 34,
        },
    ];
}
