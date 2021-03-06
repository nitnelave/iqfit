use crate::board::*;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct DisplayBoard {
    cells: [Option<Color>; 50],
}

impl Board for DisplayBoard {
    fn can_place_piece(&self, piece: PlacedPiece) -> bool {
        let info = get_placement_info(piece.piece);
        if !is_valid_piece_placement(piece, info) {
            return false;
        }
        for i in 0..info.num_balls as usize {
            let shift = info.balls[i];
            if self.cell_at(piece.top_left + shift).is_some() {
                return false;
            }
        }
        true
    }
    fn with_piece(mut self, piece: PlacedPiece) -> Self {
        let info = get_placement_info(piece.piece);
        for i in 0..info.num_balls as usize {
            let shift = info.balls[i];
            *self.mut_cell_at(piece.top_left + shift) = Some(piece.piece.color());
        }
        self
    }
    fn maybe_with_piece(&self, piece: PlacedPiece) -> Option<Self> {
        if self.can_place_piece(piece) {
            Some(self.with_piece(piece))
        } else {
            None
        }
    }
    fn is_cell_empty(&self, index: u8) -> bool {
        if index >= 50 {
            false
        } else {
            self.cells[index as usize].is_none()
        }
    }
    fn check_common_failures(&self) -> bool {
        let index = self.first_empty_cell(0);
        if index.is_none() {
            return false;
        }
        let index = index.unwrap();
        if !self.is_cell_empty(index + 10) {
            if index % 10 == 9 || !self.is_cell_empty(index + 1) {
                return true;
            }
            if !self.is_cell_empty(index + 11) && !self.is_cell_empty(index + 2) {
                return true;
            }
        }
        false
    }
    fn empty() -> Self {
        DisplayBoard { cells: [None; 50] }
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
    fn cell_at(&self, index: u8) -> &Option<Color> {
        &self.cells[index as usize]
    }

    fn mut_cell_at(&mut self, index: u8) -> &mut Option<Color> {
        &mut self.cells[index as usize]
    }
}

impl Default for DisplayBoard {
    fn default() -> Self {
        Self::empty()
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
