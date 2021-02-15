mod board;
mod pieces;
mod solver;
use board::*;
use pieces::*;

fn main() {
    // Puzzle 49.
    // let pieces = vec![
    //     PlacedPiece {
    //         piece: Piece::new()
    //             .with_color(Color::Yellow)
    //             .with_face(Face::B)
    //             .with_orientation(Orientation::Up),
    //         top_left: 10,
    //     },
    //     PlacedPiece {
    //         piece: Piece::new()
    //             .with_color(Color::Blue)
    //             .with_face(Face::A)
    //             .with_orientation(Orientation::Up),
    //         top_left: 14,
    //     },
    //     PlacedPiece {
    //         piece: Piece::new()
    //             .with_color(Color::Green)
    //             .with_face(Face::A)
    //             .with_orientation(Orientation::Up),
    //         top_left: 18,
    //     },
    // ];

    // Puzzle 117.
    let pieces = vec![
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
    let board = DisplayBoard::from_piece_list(&pieces).unwrap();
    println!("{}", board);
    let solved_board = solver::solve(board);
    if let Some(b) = solved_board {
        println!("Solving successful!");
        println!("{}", b);
    } else {
        println!("Solving failed...");
    }
}
