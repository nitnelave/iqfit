mod board;
mod pieces;
mod puzzles;
mod solver;
use board::*;

fn main() {
    let board = DisplayBoard::from_placed_piece_list(&*puzzles::PIECES_117).unwrap();
    println!("{}", board);
    let solution = solver::solve::<DisplayBoard>(&*puzzles::PIECES_117);
    if let Some(pieces) = solution {
        println!("Solving successful!");
        println!("{}", DisplayBoard::from_placed_piece_list(&pieces).unwrap());
    } else {
        println!("Solving failed...");
    }
}
