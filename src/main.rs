mod board;
mod pieces;
mod puzzles;
mod solver;
use board::*;

fn main() {
    let board = DisplayBoard::from_piece_list(&*puzzles::PIECES_117).unwrap();
    println!("{}", board);
    let solved_board = solver::solve(board);
    if let Some(b) = solved_board {
        println!("Solving successful!");
        println!("{}", b);
    } else {
        println!("Solving failed...");
    }
}
