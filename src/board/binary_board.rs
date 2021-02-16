use crate::board::*;

/// Model the board as a bitfield of empty cells.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct BinaryBoard {
    /// Bitfield of empty cells. Each bit corresponds to a specific cell, starting from the top
    /// left and going row by row.
    cells: u64,
}

impl Board for BinaryBoard {
    fn can_place_piece(&self, piece: PlacedPiece) -> bool {
        let info = get_placement_info(piece.piece);
        if !is_valid_piece_placement(piece, info) {
            return false;
        }
        let binary_piece = info.as_binary << piece.top_left;
        self.cells & binary_piece == 0
    }
    fn with_piece(mut self, piece: PlacedPiece) -> Self {
        let info = get_placement_info(piece.piece);
        let binary_piece = info.as_binary << piece.top_left;
        self.cells |= binary_piece;
        self
    }
    fn empty() -> Self {
        BinaryBoard {
            // Set the cells after the board to full.
            cells: (!0) << 50,
        }
    }

    #[inline]
    fn is_cell_empty(&self, index: u8) -> bool {
        self.cells & (1 << index) == 0
    }

    #[inline]
    fn check_common_failures(&self, index: u8) -> bool {
        const PATTERN_2: u64 = 0b110000000100;
        if !self.is_cell_empty(index + 10) && (index % 10 == 9 || !self.is_cell_empty(index + 1)) {
            return true;
        }
        (self.cells >> index) & PATTERN_2 == PATTERN_2
    }

    #[inline]
    fn first_empty_cell(&self, lower_bound: u8) -> Option<u8> {
        let first_unset_bit = self.get_first_unset_bit(lower_bound);
        if first_unset_bit >= 50 {
            None
        } else {
            Some(first_unset_bit)
        }
    }
}

impl Default for BinaryBoard {
    fn default() -> Self {
        Self::empty()
    }
}

impl BinaryBoard {
    /// Find the first bit left unset in the bitfield.
    /// This uses a lookup table to get the first unset bit in a byte efficiently.
    #[inline]
    fn get_first_unset_bit(&self, lower_bound: u8) -> u8 {
        if self.cells & (1 << lower_bound) == 0 {
            return lower_bound;
        }
        let cell_bytes = self.cells.to_le_bytes();
        for first_empty_cell_byte_index in (lower_bound / 8)..8 {
            let first_unset_bit =
                FIRST_UNSET_BIT[cell_bytes[first_empty_cell_byte_index as usize] as usize];
            if first_unset_bit < 8 {
                return first_empty_cell_byte_index * 8 + first_unset_bit;
            }
        }
        50
    }
}

include!(concat!(env!("OUT_DIR"), "/first_unset_bit_table.rs"));
