use crate::pieces::*;
use std::fmt;

mod display_board_placement_info;
mod display_board_placement_info_gen;

use display_board_placement_info::DisplayBoardPlacementInfo;

pub trait Board {
    fn place_piece(&mut self, piece: PlacedPiece) -> bool;
    fn pop_piece(&mut self);
    fn piece_list(&self) -> &Vec<PlacedPiece>;
    fn first_empty_cell(&self, lower_bound: u8) -> Option<u8>;
    fn empty() -> Self;
}

#[derive(Debug, PartialEq, Eq)]
pub struct DisplayBoard {
    placed_pieces: Vec<PlacedPiece>,
    cells: [Option<Color>; 50],
}

impl Board for DisplayBoard {
    fn place_piece(&mut self, piece: PlacedPiece) -> bool {
        let (top_left_row, top_left_col) = piece.top_left_coords();
        let info = get_placement_info(piece.piece);
        if top_left_row + info.height >= 5
            || top_left_col + info.width_right >= 10
            || top_left_col < info.width_left
        {
            return false;
        }
        for i in 0..info.num_balls as usize {
            let shift = info.balls[i];
            if self.cell_at(piece.top_left + shift).is_some() {
                return false;
            }
        }
        for i in 0..info.num_balls as usize {
            let shift = info.balls[i];
            *self.cell_at(piece.top_left + shift) = Some(piece.piece.color());
        }
        self.placed_pieces.push(piece);
        true
    }
    fn pop_piece(&mut self) {
        let piece = self.placed_pieces.pop().unwrap();
        let info = get_placement_info(piece.piece);
        for i in 0..info.num_balls as usize {
            let shift = info.balls[i];
            *self.cell_at(piece.top_left + shift) = None;
        }
    }
    fn piece_list(&self) -> &Vec<PlacedPiece> {
        &self.placed_pieces
    }
    fn empty() -> Self {
        DisplayBoard {
            placed_pieces: Vec::new(),
            cells: [None; 50],
        }
    }

    fn first_empty_cell(&self, lower_bound: u8) -> Option<u8> {
        let mut first_empty_cell_index = lower_bound;
        while first_empty_cell_index < 50 && self.cells[first_empty_cell_index as usize].is_some() {
            first_empty_cell_index += 1;
        }
        if first_empty_cell_index == 50 {
            None
        } else {
            Some(first_empty_cell_index)
        }
    }
}

impl DisplayBoard {
    fn cell_at(&mut self, index: u8) -> &mut Option<Color> {
        &mut self.cells[index as usize]
    }

    pub fn from_piece_list(pieces: &[PlacedPiece]) -> Option<Self> {
        let mut board = Self::empty();
        for p in pieces {
            if !board.place_piece(*p) {
                return None;
            }
        }
        Some(board)
    }
}

impl fmt::Display for DisplayBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use colored::Colorize;
        for row in 0..5 {
            for col in 0..10 {
                let index = row * 10 + col;
                write!(
                    f,
                    "{}",
                    match self.cells[index] {
                        None => "  ".on_black(),
                        Some(Color::Yellow) => "  ".on_yellow(),
                        Some(Color::Orange) => "  ".on_bright_red(),
                        Some(Color::Red) => "  ".on_red(),
                        Some(Color::Pink) => "  ".on_bright_purple(),
                        Some(Color::LightGreen) => "  ".on_bright_green(),
                        Some(Color::Green) => "  ".on_green(),
                        Some(Color::LightBlue) => "  ".on_bright_cyan(),
                        Some(Color::Blue) => "  ".on_bright_blue(),
                        Some(Color::DeepBlue) => "  ".on_blue(),
                        Some(Color::Purple) => "  ".on_purple(),
                    }
                )?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

const fn get_placement_info(piece: Piece) -> &'static DisplayBoardPlacementInfo {
    display_board_placement_info_gen::PLACEMENT_INFO[piece.as_byte() as usize]
}

#[cfg(test)]
mod tests {
    use crate::board::*;

    #[test]
    fn pink_display_info() {
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
            }
        );
    }

    #[test]
    fn fail_place_piece() {
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
    fn fail_place_piece_intersect() {
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
    fn success_fill_board() {
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
}
