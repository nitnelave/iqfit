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
    fn check_common_failures(&self) -> bool {
        let n = self.cells;
        // Prepare the following pattern:
        //  #
        // #.
        let base_pattern = n & (n << 9) & (!n << 10);
        // Look for the following pattern:
        //  #
        // #.#
        //  #
        let single_hole = (base_pattern & (n << 20) & (n << 11)) != 0;
        // Prepare the following pattern:
        //  ##
        // #..
        //  #
        let base_double_pattern = base_pattern & (n << 20) & (!n << 11) & (n << 1);
        // Look for the following pattern:
        //  ##
        // #..#
        //  ##
        let double_hole = (base_double_pattern & (n << 12) & (n << 21)) != 0;
        // Look for the following pattern:
        //  #
        // #.#
        // #.#
        //  #
        let double_hole_vertical = (base_pattern & (n << 11) & (!n << 20) & (n << 19) & (n << 21) & (n << 30)) != 0;
        // Look for the following pattern:
        //  ###
        // #...#
        //  ###
        let triple_hole =
            (base_double_pattern & (!n << 12) & (n << 21) & (n << 3) & (n << 22) & (n << 13)) != 0;
        // Look for the following pattern:
        //  ##
        // #..#
        //  #.#
        //   #
        let triple_hole_l =
            (base_double_pattern & (n << 12) & (!n << 21) & (n << 31) & (n << 22) & (n << 13)) != 0;
        // Look for the following pattern:
        //  ##
        // #..#
        // #.#
        //  #
        let triple_hole_l2 = (base_pattern
            & (n << 1)
            & (!n << 11)
            & (n << 12)
            & (n << 19)
            & (!n << 20)
            & (n << 21)
            & (n << 30))
            != 0;
        // Look for the following pattern:
        //  #
        // #.#
        // #..#
        //  ##
        let triple_hole_l3 = (base_pattern
            & (n << 11)
            & (n << 19)
            & (!n << 20)
            & (!n << 21)
            & (n << 22)
            & (n << 30)
            & (n << 31))
            != 0;
        // Look for the following pattern:
        //   #
        //  #.#
        // #..#
        //  ##
        let triple_hole_l4 = (base_pattern
            & (n << 11)
            & (n << 18)
            & (!n << 19)
            & (!n << 20)
            & (n << 21)
            & (n << 29)
            & (n << 30))
            != 0;
        single_hole
            || double_hole
            || double_hole_vertical
            || triple_hole
            || triple_hole_l
            || triple_hole_l2
            || triple_hole_l3
            || triple_hole_l4
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
