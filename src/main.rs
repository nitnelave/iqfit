mod board;
mod pieces;
use crate::pieces::*;

fn main() {
    let piece = Piece::new()
        .with_color(Color::Yellow)
        .with_face(Face::A)
        .with_orientation(Orientation::Right);
    println!(
        "Piece: {:?} ({:?} with size {} bytes)",
        piece,
        piece.as_byte(),
        std::mem::size_of::<Piece>()
    );
    let c = Piece::from_byte(38).unwrap().color() as u8;
    println!("Color: {}", c);
    for n in 8..16 {
        println!("Piece: {:?}", Piece::from_byte(n).unwrap());
    }
}
