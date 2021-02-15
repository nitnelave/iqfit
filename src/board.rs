use crate::pieces::*;

pub mod binary_board;
pub mod display_board;
mod display_board_placement_info;
mod display_board_placement_info_gen;

pub use binary_board::BinaryBoard;
pub use display_board::DisplayBoard;

use display_board_placement_info::DisplayBoardPlacementInfo;

pub trait Board {
    fn place_piece(&mut self, piece: PlacedPiece) -> bool;
    fn pop_piece(&mut self);
    fn piece_list(&self) -> &Vec<PlacedPiece>;
    fn first_empty_cell(&self, lower_bound: u8) -> Option<u8>;
    fn empty() -> Self;
}

const fn get_placement_info(piece: Piece) -> &'static DisplayBoardPlacementInfo {
    display_board_placement_info_gen::PLACEMENT_INFO[piece.as_byte() as usize]
}

const fn is_valid_piece_placement(piece: PlacedPiece) -> bool {
    let (top_left_row, top_left_col) = piece.top_left_coords();
    let info = get_placement_info(piece.piece);
    top_left_row + info.height < 5
        && top_left_col + info.width_right < 10
        && top_left_col >= info.width_left
}

#[cfg(test)]
#[generic_tests::define]
mod tests {
    use crate::board::*;

    #[test]
    fn pink_display_info<B>()
    where
        B: Board,
    {
        let info = get_placement_info(
            Piece::new()
                .with_color(Color::Pink)
                .with_face(Face::A)
                .with_orientation(Orientation::Right),
        );
        assert_eq!(
            *info,
            DisplayBoardPlacementInfo {
                height: 1,
                width_right: 3,
                width_left: 0,
                num_balls: 5,
                balls: [0, 1, 2, 3, 12, 0],
                as_binary: 0b1000000001111,
            }
        );
    }

    #[test]
    fn fail_place_piece<B>()
    where
        B: Board,
    {
        let mut board = DisplayBoard::empty();
        // Too far right.
        assert!(!board.place_piece(PlacedPiece {
            piece: Piece::new()
                .with_color(Color::Pink)
                .with_face(Face::A)
                .with_orientation(Orientation::Right),
            top_left: 7,
        }));
        // Too low.
        assert!(!board.place_piece(PlacedPiece {
            piece: Piece::new()
                .with_color(Color::Pink)
                .with_face(Face::A)
                .with_orientation(Orientation::Right),
            top_left: 42,
        }));
        // Too far left.
        assert!(!board.place_piece(PlacedPiece {
            piece: Piece::new()
                .with_color(Color::Pink)
                .with_face(Face::A)
                .with_orientation(Orientation::Left),
            top_left: 20,
        }));

        assert_eq!(board, DisplayBoard::empty());
    }

    #[test]
    fn fail_place_piece_intersect<B>()
    where
        B: Board,
    {
        let mut board = DisplayBoard::empty();
        assert!(board.place_piece(PlacedPiece {
            piece: Piece::new()
                .with_color(Color::Pink)
                .with_face(Face::A)
                .with_orientation(Orientation::Right),
            top_left: 22,
        }));
        // Intersects.
        assert!(!board.place_piece(PlacedPiece {
            piece: Piece::new()
                .with_color(Color::Yellow)
                .with_face(Face::B)
                .with_orientation(Orientation::Up),
            top_left: 11,
        }));

        // Intersects.
        assert!(!board.place_piece(PlacedPiece {
            piece: Piece::new()
                .with_color(Color::Yellow)
                .with_face(Face::B)
                .with_orientation(Orientation::Up),
            top_left: 4,
        }));

        assert_eq!(board.piece_list().len(), 1);

        // Doesn't intersect.
        assert!(board.place_piece(PlacedPiece {
            piece: Piece::new()
                .with_color(Color::Yellow)
                .with_face(Face::B)
                .with_orientation(Orientation::Up),
            top_left: 0,
        }));

        assert_eq!(board.piece_list().len(), 2);
    }

    #[test]
    fn success_fill_board<B>()
    where
        B: Board,
    {
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
        assert_eq!(*board.piece_list(), pieces);
        let new_board = DisplayBoard::from_piece_list(&pieces);
        assert!(new_board.is_some());
        assert_eq!(board, new_board.unwrap());
    }

    #[instantiate_tests(<DisplayBoard>)]
    mod display_board {}
    #[instantiate_tests(<BinaryBoard>)]
    mod binary_board {}
}
