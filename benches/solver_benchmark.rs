use criterion::{criterion_group, criterion_main, Criterion};
use iqfit_solver::board::BinaryBoard;
use iqfit_solver::pieces::*;
use iqfit_solver::puzzles::*;
use iqfit_solver::solver;

fn solve(pieces: &[PlacedPiece]) -> Vec<PlacedPiece> {
    solver::solve::<BinaryBoard>(&pieces).unwrap()
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("solve 49", |b| b.iter(|| solve(&*PIECES_49)));
    c.bench_function("solve 117", |b| b.iter(|| solve(&*PIECES_117)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
