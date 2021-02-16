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

fn solve_rec<B: Board, C: IterationCounter>(
    board: &mut B,
    colors_left: ColorSet,
    empty_index_lower_bound: u8,
    num_face_a: u8,
    num_face_b: u8,
    counter: &mut C,
) -> bool {
    let index = board.first_empty_cell(empty_index_lower_bound);
    if index.is_none() {
        return true;
    }
    let index = index.unwrap();
    if !board.is_cell_empty(index + 1) && !board.is_cell_empty(index + 10) {
        return false;
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
            if *face == Face::A {
                if num_face_a == 6 {
                    continue;
                }
            } else if num_face_b == 4 {
                continue;
            }
            piece.piece.set_face(*face);
            for orientation in ORIENTATION_LIST.iter() {
                piece.piece.set_orientation(*orientation);
                counter.increment();
                if board.place_piece(piece) {
                    let num_a = num_face_a + (*face == Face::A) as u8;
                    let num_b = num_face_b + (*face == Face::B) as u8;
                    if solve_rec(
                        board,
                        next_colors,
                        index + 1,
                        num_a,
                        num_b,
                        counter,
                    ) {
                        return true;
                    }
                    board.pop_piece();
                }
            }
        }
    }
    false
}

fn solve_impl<B: Board, C: IterationCounter>(mut board: B, counter: &mut C) -> Option<B> {
    let mut colors_left = ColorSet::full();
    for p in board.piece_list() {
        assert!(colors_left.remove(p.piece.color()));
    }
    let num_face_a = board
        .piece_list()
        .iter()
        .filter(|p| p.piece.face() == Face::A)
        .count() as u8;
    let num_face_b = board.piece_list().len() as u8 - num_face_a;

    if solve_rec(&mut board, colors_left, 0, num_face_a, num_face_b, counter) {
        Some(board)
    } else {
        None
    }
}

pub fn solve<B: Board>(board: B) -> Option<B> {
    let mut counter = NoOpIterationCounter {};
    solve_impl(board, &mut counter)
}

#[cfg(test)]
pub fn solve_with_counter<B: Board>(board: B) -> (Option<B>, u64) {
    let mut counter = SimpleIterationCounter(0);
    let b = solve_impl(board, &mut counter);
    (b, counter.get())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::BinaryBoard;
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
        let board = BinaryBoard::from_piece_list(&*PIECES_49).unwrap();
        let (b, c) = solve_with_counter(board);
        assert!(b.is_some());
        assert_eq!(b.unwrap().first_empty_cell(0), None);
        assert_eq!(c, 787);
    }

    #[test]
    fn test_117() {
        let board = BinaryBoard::from_piece_list(&*PIECES_117).unwrap();
        let (b, c) = solve_with_counter(board);
        assert!(b.is_some());
        assert_eq!(b.unwrap().first_empty_cell(0), None);
        assert_eq!(c, 3043896);
    }
}
