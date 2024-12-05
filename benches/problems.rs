use advent2024::day01::Day01;
use advent2024::day02::Day02;
use advent2024::day03::Day03;
use advent2024::day05::Day05;
use advent2024::problem::ProblemLines;

use criterion::{criterion_group, criterion_main, Criterion};

pub fn day01(c: &mut Criterion) {
    let file = std::fs::read_to_string("inputs/input01.txt").unwrap();
    let problem = Box::from(Day01 {});
    c.bench_function("Day 01 Part 1", |b| b.iter(|| problem.part1(file.lines())));
    c.bench_function("Day 01 Part 2", |b| b.iter(|| problem.part2(file.lines())));
}

pub fn day02(c: &mut Criterion) {
    let file = std::fs::read_to_string("inputs/input02.txt").unwrap();
    let problem = Box::from(Day02 {});
    c.bench_function("Day 02 Part 1", |b| b.iter(|| problem.part1(file.lines())));
    c.bench_function("Day 02 Part 2", |b| b.iter(|| problem.part2(file.lines())));
}

pub fn day03(c: &mut Criterion) {
    let file = std::fs::read_to_string("inputs/input03.txt").unwrap();
    let problem = Box::from(Day03::new());
    c.bench_function("Day 03 Part 1", |b| b.iter(|| problem.part1(file.lines())));
    c.bench_function("Day 03 Part 2", |b| b.iter(|| problem.part2(file.lines())));
}
pub fn day05(c: &mut Criterion) {
    let file = std::fs::read_to_string("inputs/input05.txt").unwrap();
    let problem = Box::from(Day05 {});
    c.bench_function("Day 05 Part 1", |b| b.iter(|| problem.part1(file.lines())));
    c.bench_function("Day 05 Part 2", |b| b.iter(|| problem.part2(file.lines())));
}

criterion_group!(name = benches; config = Criterion::default().sample_size(100000); targets = day05);
criterion_main!(benches);
