use std::str::Lines;

pub trait Problem {
    fn part1(&self, lines: Lines) -> String;
    fn part2(&self, lines: Lines) -> String;
}
