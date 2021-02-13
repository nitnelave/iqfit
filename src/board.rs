use crate::pieces::*;

mod display_board_placement_info_gen;

trait Board {
    fn place_piece(&mut self, piece: PlacedPiece) -> bool;
    fn pop_piece(&mut self);
    fn piece_list(&self) -> &Vec<PlacedPiece>;
    fn empty() -> Self;
    fn from_piece_list(pieces: &Vec<PlacedPiece>) -> Option<Self>
    where
        Self: Sized,
    {
        let mut board = Self::empty();
        for p in pieces {
            if !board.place_piece(*p) {
                return None;
            }
        }
        Some(board)
    }
}

#[derive(Debug)]
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
}

impl DisplayBoard {
    fn cell_at(&mut self, index: u8) -> &mut Option<Color> {
        &mut self.cells[index as usize]
    }
}

pub struct DisplayBoardPlacementInfo {
    /// How many columns to the right of the top_left do you need.
    pub width_right: u8,
    /// How many columns to the left of the top_left do you need.
    pub width_left: u8,
    /// How many rows below the top_left do you need.
    pub height: u8,

    pub num_balls: u8,
    pub balls: [u8; 6],
}

pub const fn get_placement_info(piece: Piece) -> &'static DisplayBoardPlacementInfo {
    display_board_placement_info_gen::PLACEMENT_INFO[piece.as_byte() as usize]
}
