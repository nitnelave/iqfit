use crate::pieces::*;

pub mod binary_board;
pub mod display_board;
mod display_board_placement_info;
mod display_board_placement_info_gen;

pub use binary_board::BinaryBoard;
pub use display_board::DisplayBoard;

use display_board_placement_info::DisplayBoardPlacementInfo;

/// Represents a board on which you can place pieces.
pub trait Board: Sized + Copy + Default {
    /// Try to place a piece and return whether it succeeded.
    /// If it fails the board is left as-is.
    /// If it succeeds, the board is updated with the piece, and the piece is added to the
    /// piece_list.
    fn can_place_piece(&self, piece: PlacedPiece) -> bool;
    fn with_piece(self, piece: PlacedPiece) -> Self;
    /// Try to place a piece in the first empty spot in the top left and return whether it
    /// succeeded.
    fn place_piece_top_left(&mut self, piece: Piece) -> bool {
        if let Some(index) = self.first_empty_cell(0) {
            let placed_piece = PlacedPiece {
                top_left: index,
                piece,
            };
            if self.can_place_piece(placed_piece) {
                *self = self.with_piece(placed_piece);
                true
            } else {
                false
            }
        } else {
            false
        }
    }
    /// Find the first cell that hasn't been covered by a piece yet.
    /// `lower_bound` is the first cell that might be empty, indexed from the top left.
    fn first_empty_cell(&self, lower_bound: u8) -> Option<u8>;
    /// Check if a given cell is empty;
    fn is_cell_empty(&self, index: u8) -> bool;
    /// Check for common failure patterns.
    fn check_common_failures(&self) -> bool;
    /// Create an empty board.
    fn empty() -> Self;
    fn from_piece_list(pieces: &[Piece]) -> Option<Self> {
        let mut board = Self::default();
        for p in pieces {
            if !board.place_piece_top_left(*p) {
                return None;
            }
        }
        Some(board)
    }
    fn from_placed_piece_list(pieces: &[PlacedPiece]) -> Option<Self> {
        let mut board = Self::empty();
        for p in pieces {
            if !board.can_place_piece(*p) {
                return None;
            }
            board = board.with_piece(*p);
        }
        Some(board)
    }
}

#[inline]
const fn get_placement_info(piece: Piece) -> &'static DisplayBoardPlacementInfo {
    display_board_placement_info_gen::PLACEMENT_INFO[piece.as_byte() as usize]
}

#[inline]
const fn is_valid_piece_placement(piece: PlacedPiece, info: &DisplayBoardPlacementInfo) -> bool {
    let (top_left_row, top_left_col) = piece.top_left_coords();
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
        let board = DisplayBoard::empty();
        // Too far right.
        assert!(!board.can_place_piece(PlacedPiece {
            piece: Piece::new()
                .with_color(Color::Pink)
                .with_face(Face::A)
                .with_orientation(Orientation::Right),
            top_left: 7,
        }));
        // Too low.
        assert!(!board.can_place_piece(PlacedPiece {
            piece: Piece::new()
                .with_color(Color::Pink)
                .with_face(Face::A)
                .with_orientation(Orientation::Right),
            top_left: 42,
        }));
        // Too far left.
        assert!(!board.can_place_piece(PlacedPiece {
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
        let pink_piece = PlacedPiece {
            piece: Piece::new()
                .with_color(Color::Pink)
                .with_face(Face::A)
                .with_orientation(Orientation::Right),
            top_left: 22,
        };
        assert!(board.can_place_piece(pink_piece));
        board = board.with_piece(pink_piece);
        // Intersects.
        assert!(!board.can_place_piece(PlacedPiece {
            piece: Piece::new()
                .with_color(Color::Yellow)
                .with_face(Face::B)
                .with_orientation(Orientation::Up),
            top_left: 11,
        }));

        // Intersects.
        assert!(!board.can_place_piece(PlacedPiece {
            piece: Piece::new()
                .with_color(Color::Yellow)
                .with_face(Face::B)
                .with_orientation(Orientation::Up),
            top_left: 4,
        }));

        // Doesn't intersect.
        assert!(board.can_place_piece(PlacedPiece {
            piece: Piece::new()
                .with_color(Color::Yellow)
                .with_face(Face::B)
                .with_orientation(Orientation::Up),
            top_left: 0,
        }));
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
            assert!(board.can_place_piece(*p));
            board = board.with_piece(*p);
        }
        let new_board = DisplayBoard::from_placed_piece_list(&pieces);
        assert!(new_board.is_some());
        assert_eq!(board, new_board.unwrap());
    }

    #[instantiate_tests(<DisplayBoard>)]
    mod display_board {}
    #[instantiate_tests(<BinaryBoard>)]
    mod binary_board {}
}
