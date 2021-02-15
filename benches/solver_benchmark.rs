use criterion::{criterion_group, criterion_main, Criterion};
use iqfit_solver::board::DisplayBoard;
use iqfit_solver::pieces::*;
use iqfit_solver::solver;
use lazy_static::lazy_static;

lazy_static! {
    static ref PIECES_49: [PlacedPiece; 3] = [
        PlacedPiece {
            piece: Piece::new()
                .with_color(Color::Yellow)
                .with_face(Face::B)
                .with_orientation(Orientation::Up),
            top_left: 10,
        },
        PlacedPiece {
            piece: Piece::new()
                .with_color(Color::Blue)
                .with_face(Face::A)
                .with_orientation(Orientation::Up),
            top_left: 14,
        },
        PlacedPiece {
            piece: Piece::new()
                .with_color(Color::Green)
                .with_face(Face::A)
                .with_orientation(Orientation::Up),
            top_left: 18,
        },
    ];
    static ref PIECES_117: [PlacedPiece; 2] = [
        PlacedPiece {
            piece: Piece::new()
                .with_color(Color::LightBlue)
                .with_face(Face::B)
                .with_orientation(Orientation::Right),
            top_left: 3,
        },
        PlacedPiece {
            piece: Piece::new()
                .with_color(Color::DeepBlue)
                .with_face(Face::A)
                .with_orientation(Orientation::Left),
            top_left: 34,
        },
    ];
}

fn solve(pieces: &[PlacedPiece]) -> DisplayBoard {
    let board = DisplayBoard::from_piece_list(&pieces).unwrap();
    solver::solve(board).unwrap()
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("solve 49", |b| b.iter(|| solve(&*PIECES_49)));
    c.bench_function("solve 117", |b| b.iter(|| solve(&*PIECES_117)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
