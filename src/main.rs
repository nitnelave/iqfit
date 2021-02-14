mod board;
mod pieces;
use pieces::*;
use board::*;

fn main() {
    let piece = Piece::new()
        .with_color(Color::Yellow)
        .with_face(Face::A)
        .with_orientation(Orientation::Right);
    println!(
        "Piece: {:?} ({:?} with size {} bytes)",
        piece,
        piece.as_byte(),
        std::mem::size_of::<Piece>()
    );
    let c = Piece::from_byte(38).unwrap().color() as u8;
    println!("Color: {}", c);
    for n in 8..16 {
        println!("Piece: {:?}", Piece::from_byte(n).unwrap());
    }

    let mut board = DisplayBoard::empty();
    // Solution 49.
    let pieces = vec![
        PlacedPiece {
            piece: Piece::new()
                .with_color(Color::Pink)
                .with_face(Face::A)
                .with_orientation(Orientation::Right),
            top_left: 0,
        },
        PlacedPiece {
            piece: Piece::new()
                .with_color(Color::Red)
                .with_face(Face::A)
                .with_orientation(Orientation::Down),
            top_left: 4,
        },
        PlacedPiece {
            piece: Piece::new()
                .with_color(Color::LightBlue)
                .with_face(Face::A)
                .with_orientation(Orientation::Up),
            top_left: 6,
        },
        PlacedPiece {
            piece: Piece::new()
                .with_color(Color::LightGreen)
                .with_face(Face::B)
                .with_orientation(Orientation::Right),
            top_left: 7,
        },
        PlacedPiece {
            piece: Piece::new()
                .with_color(Color::Yellow)
                .with_face(Face::B)
                .with_orientation(Orientation::Up),
            top_left: 10,
        },
        PlacedPiece {
            piece: Piece::new()
                .with_color(Color::DeepBlue)
                .with_face(Face::A)
                .with_orientation(Orientation::Down),
            top_left: 13,
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
        PlacedPiece {
            piece: Piece::new()
                .with_color(Color::Purple)
                .with_face(Face::B)
                .with_orientation(Orientation::Left),
            top_left: 31,
        },
        PlacedPiece {
            piece: Piece::new()
                .with_color(Color::Orange)
                .with_face(Face::B)
                .with_orientation(Orientation::Left),
            top_left: 37,
        },
    ];
    for p in pieces.iter() {
        assert!(board.place_piece(*p));
    }
    println!("{}", board);
}
