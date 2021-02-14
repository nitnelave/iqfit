use modular_bitfield::error::OutOfBounds;
use modular_bitfield::{bitfield, BitfieldSpecifier};

/// Which physical piece.
#[derive(BitfieldSpecifier, Debug, Copy, Clone, PartialEq, Eq)]
#[bits = 4]
pub enum Color {
    Yellow,
    Orange,
    Red,
    Pink,
    LightGreen,
    Green,
    LightBlue,
    Blue,
    DeepBlue,
    Purple,
}

/// Which face:
#[derive(BitfieldSpecifier, Debug, Copy, Clone, PartialEq, Eq)]
pub enum Face {
    /// One ball sticking out.
    A,
    /// Two balls.
    B,
}

/// Which way the piece is facing.
/// Up has the main line vertical, and the extra ball(s) to the right.
/// The rest are successive 90 degrees rotations to the right.
#[derive(BitfieldSpecifier, Debug, Copy, Clone, PartialEq, Eq)]
pub enum Orientation {
    Up,
    Right,
    Down,
    Left,
}

#[bitfield(filled = false)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Piece {
    pub orientation: Orientation,
    pub face: Face,
    pub color: Color,
}

impl Piece {
    pub const fn as_byte(&self) -> u8 {
        self.into_bytes()[0]
    }
    pub fn from_byte(b: u8) -> Result<Piece, OutOfBounds> {
        Piece::from_bytes([b])
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct PlacedPiece {
    /// Which piece, in which orientation.
    pub piece: Piece,
    /// The position of the top-left corner, from 0 to 50.
    pub top_left: u8,
}

impl PlacedPiece {
    pub fn top_left_coords(&self) -> (u8, u8) {
        (self.top_left / 10, self.top_left % 10)
    }
}
