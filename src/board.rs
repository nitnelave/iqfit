use crate::pieces::*;

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
    cells: [[Option<Color>; 5]; 10],
}

struct DisplayBoardPlacementInfo {
    pub num_balls: u8,
    pub width: u8,
    pub height: u8,
    pub balls: [(u8, u8); 6],
}

impl Board for DisplayBoard {
    fn place_piece(&mut self, piece: PlacedPiece) -> bool {
        let (top_left_row, top_left_col) = piece.top_left_coords();
        let info = DisplayBoard::get_placement_info(piece.piece);
        if top_left_row + info.height > 5 || top_left_col + info.width > 10 {
            return false;
        }
        for i in 0..info.num_balls as usize {
            let (row, col) = info.balls[i];
            if self.cell_at(top_left_row + row, top_left_col + col).is_some() {
                return false;
            }
        }
        for i in 0..info.num_balls  as usize{
            let (row, col) = info.balls[i];
            *self.cell_at(top_left_row + row, top_left_col + col) = Some(piece.piece.color());
        }
        self.placed_pieces.push(piece);
        true
    }
    fn pop_piece(&mut self) {
        let piece = self.placed_pieces.pop().unwrap();
        let (top_left_row, top_left_col) = piece.top_left_coords();
        let info = DisplayBoard::get_placement_info(piece.piece);
        for i in 0..info.num_balls  as usize{
            let (row, col) = info.balls[i];
            *self.cell_at(top_left_row + row, top_left_col + col) = None;
        }
    }
    fn piece_list(&self) -> &Vec<PlacedPiece> {
        &self.placed_pieces
    }
    fn empty() -> Self {
        DisplayBoard {
            placed_pieces: Vec::new(),
            cells: [[None; 5]; 10],
        }
    }
}

impl DisplayBoard {
    fn get_placement_info(_piece: Piece) -> DisplayBoardPlacementInfo {
        unimplemented!()
    }

    fn cell_at(&mut self, row: u8, col: u8) -> &mut Option<Color> {
        &mut self.cells[row as usize][col as usize]
    }
}
