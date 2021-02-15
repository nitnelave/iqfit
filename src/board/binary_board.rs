use crate::board::*;

#[derive(Debug, PartialEq, Eq)]
pub struct BinaryBoard {
    placed_pieces: Vec<PlacedPiece>,
    cells: u64,
}

impl Board for BinaryBoard {
    fn place_piece(&mut self, piece: PlacedPiece) -> bool {
        if !is_valid_piece_placement(piece) {
            return false;
        }
        let info = get_placement_info(piece.piece);
        let binary_piece = info.as_binary << piece.top_left;
        if self.cells & binary_piece != 0 {
            return false;
        }
        self.cells |= binary_piece;
        self.placed_pieces.push(piece);
        true
    }
    fn pop_piece(&mut self) {
        let piece = self.placed_pieces.pop().unwrap();
        let info = get_placement_info(piece.piece);
        let binary_piece = info.as_binary << piece.top_left;
        self.cells &= !binary_piece;
    }
    fn piece_list(&self) -> &Vec<PlacedPiece> {
        &self.placed_pieces
    }
    fn empty() -> Self {
        BinaryBoard {
            placed_pieces: Vec::new(),
            cells: 0,
        }
    }

    fn first_empty_cell(&self, lower_bound: u8) -> Option<u8> {
        let mut first_empty_cell_index = lower_bound;
        while first_empty_cell_index < 50
            && (self.cells & (1 << first_empty_cell_index as u64) != 0)
        {
            first_empty_cell_index += 1;
        }
        if first_empty_cell_index == 50 {
            None
        } else {
            Some(first_empty_cell_index)
        }
    }
}

impl BinaryBoard {
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
