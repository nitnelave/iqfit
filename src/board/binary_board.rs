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
        let first_unset_bit = self.get_first_unset_bit(lower_bound);
        if first_unset_bit == 50 {
            None
        } else {
            Some(first_unset_bit)
        }
    }
}

impl BinaryBoard {
    #[allow(dead_code)]
    pub fn from_piece_list(pieces: &[PlacedPiece]) -> Option<Self> {
        let mut board = Self::empty();
        for p in pieces {
            if !board.place_piece(*p) {
                return None;
            }
        }
        Some(board)
    }

    fn get_first_unset_bit(&self, lower_bound: u8) -> u8 {
        let mut first_empty_cell_byte_index = lower_bound / 8;
        if self.cells & (1 << lower_bound) == 0 {
            return lower_bound;
        }
        let cell_bytes = self.cells.to_le_bytes();
        loop {
            let first_unset_bit =
                FIRST_UNSET_BIT[cell_bytes[first_empty_cell_byte_index as usize] as usize];
            if first_unset_bit < 8 {
                return first_empty_cell_byte_index * 8 + first_unset_bit;
            }
            assert!(
                first_empty_cell_byte_index < 7,
                "Could not find first empty cell!"
            );
            first_empty_cell_byte_index += 1;
        }
    }
}

include!(concat!(env!("OUT_DIR"), "/first_unset_bit_table.rs"));
