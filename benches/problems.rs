use advent2024::day01::Day01;
use advent2024::problem::Problem;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn problems(c: &mut Criterion) {
    let file = std::fs::read_to_string("inputs/input01.txt").unwrap();
    let problem = Box::from(Day01 {});
    c.bench_function("Day 01 Part 1", |b| b.iter(|| problem.part1(file.lines())));
}

criterion_group!(name = benches; config = Criterion::default().sample_size(10000); targets = problems);
criterion_main!(benches);
