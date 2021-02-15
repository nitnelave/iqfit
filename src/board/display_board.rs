use crate::board::*;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct DisplayBoard {
    placed_pieces: Vec<PlacedPiece>,
    cells: [Option<Color>; 50],
}

impl Board for DisplayBoard {
    fn place_piece(&mut self, piece: PlacedPiece) -> bool {
        if !is_valid_piece_placement(piece) {
            return false;
        }
        let info = get_placement_info(piece.piece);
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
            writeln!(f)?;
        }
        Ok(())
    }
}
