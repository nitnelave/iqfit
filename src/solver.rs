use crate::board::Board;
use crate::pieces::*;

const COLOR_LIST: [Color; 10] = [
    Color::Yellow,
    Color::Orange,
    Color::Red,
    Color::Pink,
    Color::LightGreen,
    Color::Green,
    Color::LightBlue,
    Color::Blue,
    Color::DeepBlue,
    Color::Purple,
];

const FACE_LIST: [Face; 2] = [Face::A, Face::B];

const ORIENTATION_LIST: [Orientation; 4] = [
    Orientation::Up,
    Orientation::Right,
    Orientation::Down,
    Orientation::Left,
];

pub trait IterationCounter {
    fn increment(&mut self);
    fn get(&self) -> u64;
}

struct NoOpIterationCounter {}

impl IterationCounter for NoOpIterationCounter {
    #[inline]
    fn increment(&mut self) {}
    #[inline]
    fn get(&self) -> u64 {
        0
    }
}

struct SimpleIterationCounter(u64);

impl IterationCounter for SimpleIterationCounter {
    #[inline]
    fn increment(&mut self) {
        self.0 += 1;
        if self.0 > 5000000 {
            panic!("Too many iterations!");
        }
    }
    #[inline]
    fn get(&self) -> u64 {
        self.0
    }
}

/// A set of colors, starting full, and getting progressively empty.
#[derive(Copy, Clone)]
struct ColorSet(pub u16);

impl ColorSet {
    pub fn full() -> Self {
        ColorSet(!0)
    }

    pub fn remove(&mut self, c: Color) -> bool {
        let res = self.contains(c);
        self.0 &= !(1 << (c as u8));
        res
    }

    pub fn contains(&self, c: Color) -> bool {
        (self.0 & 1 << (c as u8)) != 0
    }

    pub fn without_color(mut self, c: Color) -> Self {
        self.remove(c);
        self
    }
}

trait FacePolicy: Copy {
    fn can_add_face(&self, f: Face) -> bool;
    fn with_face(self, f: Face) -> Self;
    fn from_placed_pieces(pieces: &[PlacedPiece]) -> Self;
}

#[derive(Copy, Clone)]
struct TenPieceFacePolicy {
    num_face_a: u8,
    num_face_b: u8,
}

impl FacePolicy for TenPieceFacePolicy {
    #[inline]
    fn can_add_face(&self, f: Face) -> bool {
        match f {
            Face::A => {
                if self.num_face_a == 6 {
                    return false;
                }
            }
            Face::B => {
                if self.num_face_b == 4 {
                    return false;
                }
            }
        };
        true
    }
    #[inline]
    fn with_face(mut self, f: Face) -> Self {
        self.num_face_a += (f == Face::A) as u8;
        self.num_face_b += (f == Face::B) as u8;
        self
    }
    fn from_placed_pieces(pieces: &[PlacedPiece]) -> Self {
        let num_face_a = pieces.iter().filter(|p| p.piece.face() == Face::A).count() as u8;
        let num_face_b = pieces.len() as u8 - num_face_a;
        TenPieceFacePolicy {
            num_face_a,
            num_face_b,
        }
    }
}

fn solve_rec<B: Board + Copy, C: IterationCounter, F: FacePolicy>(
    board: B,
    colors_left: ColorSet,
    empty_index_lower_bound: u8,
    face_policy: F,
    counter: &mut C,
) -> Option<Vec<PlacedPiece>> {
    let index = board.first_empty_cell(empty_index_lower_bound);
    if index.is_none() {
        return Some(Vec::with_capacity(10));
    }
    let index = index.unwrap();
    if board.check_common_failures(index) {
        return None;
    }
    let mut piece = PlacedPiece {
        piece: Piece::new(),
        top_left: index,
    };
    for c in COLOR_LIST.iter() {
        if !colors_left.contains(*c) {
            continue;
        }
        let next_colors = colors_left.without_color(*c);
        piece.piece.set_color(*c);
        for face in FACE_LIST.iter() {
            if !face_policy.can_add_face(*face) {
                continue;
            }
            piece.piece.set_face(*face);
            for orientation in ORIENTATION_LIST.iter() {
                piece.piece.set_orientation(*orientation);
                counter.increment();
                if board.can_place_piece(piece) {
                    if let Some(mut pieces) = solve_rec(
                        board.with_piece(piece),
                        next_colors,
                        index + 1,
                        face_policy.with_face(*face),
                        counter,
                    ) {
                        pieces.push(piece);
                        return Some(pieces);
                    }
                }
            }
        }
    }
    None
}

fn solve_impl<B: Board, C: IterationCounter, F: FacePolicy>(
    pieces: &[PlacedPiece],
    counter: &mut C,
) -> Option<Vec<PlacedPiece>> {
    let mut colors_left = ColorSet::full();
    let mut board = B::default();
    for p in pieces {
        debug_assert!(colors_left.remove(p.piece.color()));
        debug_assert!(board.can_place_piece(*p));
        board = board.with_piece(*p);
    }
    if let Some(mut pieces_solution) = solve_rec(
        board,
        colors_left,
        0,
        F::from_placed_pieces(pieces),
        counter,
    ) {
        for p in pieces {
            pieces_solution.push(*p);
        }
        Some(pieces_solution)
    } else {
        None
    }
}

pub fn solve<B: Board>(pieces: &[PlacedPiece]) -> Option<Vec<PlacedPiece>> {
    let mut counter = NoOpIterationCounter {};
    solve_impl::<B, NoOpIterationCounter, TenPieceFacePolicy>(pieces, &mut counter)
}

#[cfg(test)]
pub fn solve_with_counter<B: Board>(pieces: &[PlacedPiece]) -> (Option<Vec<PlacedPiece>>, u64) {
    let mut counter = SimpleIterationCounter(0);
    let b = solve_impl::<B, SimpleIterationCounter, TenPieceFacePolicy>(pieces, &mut counter);
    (b, counter.get())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::{BinaryBoard, DisplayBoard};
    use crate::puzzles::*;

    #[test]
    fn test_color_to_int() {
        assert_eq!(Color::Yellow as u8, 0);
        assert_eq!(Color::Pink as u8, 3);
    }

    #[test]
    fn test_color_hasher() {
        let mut set = ColorSet::full();
        assert_eq!(set.0, 0b1111111111111111);
        assert_eq!(set.without_color(Color::Pink).0, 0b1111111111110111);
        assert_eq!(set.0, 0b1111111111111111);
        set.remove(Color::Pink);
        assert_eq!(set.0, 0b1111111111110111);
        assert!(set.contains(Color::Yellow));
        assert!(!set.contains(Color::Pink));
    }

    #[test]
    fn test_49() {
        let (pieces, c) = solve_with_counter::<BinaryBoard>(&*PIECES_49);
        assert!(pieces.is_some());
        assert_eq!(
            BinaryBoard::from_placed_piece_list(&pieces.clone().unwrap())
                .unwrap()
                .first_empty_cell(0),
            None
        );
        assert_eq!(
            DisplayBoard::from_placed_piece_list(&pieces.unwrap())
                .unwrap()
                .first_empty_cell(0),
            None
        );
        assert_eq!(c, 471);
    }

    #[test]
    fn test_117() {
        let (pieces, c) = solve_with_counter::<BinaryBoard>(&*PIECES_117);
        assert!(pieces.is_some());
        assert_eq!(
            BinaryBoard::from_placed_piece_list(&pieces.clone().unwrap())
                .unwrap()
                .first_empty_cell(0),
            None
        );
        assert_eq!(
            DisplayBoard::from_placed_piece_list(&pieces.unwrap())
                .unwrap()
                .first_empty_cell(0),
            None
        );
        assert_eq!(c, 932400);
    }
}
