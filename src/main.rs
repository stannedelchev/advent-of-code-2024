mod day01;
mod day02;
mod day03;
mod day05;
mod problem;

use crate::problem::Problem;
use day01::Day01;
use day02::Day02;
use day03::Day03;
use day05::Day05;

fn main() {
    let day03 = Day03::new();
    let problems: Vec<(Problem, &str, &str, &str, &str)> = vec![
        (
            Problem::Lines(Box::from(Day01 {})),
            "Day01",
            "inputs/input01.txt",
            "765748",
            "27732508",
        ),
        (
            Problem::Lines(Box::new(Day02 {})),
            "Day02",
            "inputs/input02.txt",
            "334",
            "400",
        ),
        (
            Problem::Lines(Box::new(day03)),
            "Day03",
            "inputs/input03.txt",
            "188741603",
            "67269798",
        ),
        (
            Problem::Lines(Box::new(Day05 {})),
            "Day05",
            "inputs/input05.txt",
            "6051",
            "5093",
        ),
    ];

    for (problem, name, path, part1_solution, part2_solution) in problems {
        let file = std::fs::read_to_string(path).unwrap();

        let part1 = match &problem {
            Problem::Lines(p) => p.part1(file.lines()),
            Problem::String(p) => p.part1(&file),
        };
        let part1_correct = part1 == part1_solution;
        println!(
            "{} part 1: {} {}",
            name,
            part1,
            checkmark_or_cross(part1_correct)
        );

        let part2 = match &problem {
            Problem::Lines(p) => p.part2(file.lines()),
            Problem::String(p) => p.part2(&file),
        };
        let part2_correct = part2 == part2_solution;
        println!(
            "{} part 2: {} {}",
            name,
            part2,
            checkmark_or_cross(part2_correct)
        );
    }
}

fn checkmark_or_cross(value: bool) -> char {
    if value {
        '\u{2705}'
    } else {
        '\u{274C}'
    }
}
