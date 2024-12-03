mod day01;
mod day02;
mod day03;
mod problem;

use day01::Day01;
use day02::Day02;
use day03::Day03;
use problem::Problem;

fn main() {
    let day03 = Day03::new();
    let problems: Vec<(&dyn Problem, &str, &str, &str, &str)> = vec![
        (
            &Day01 {},
            "Day01",
            "inputs/input01.txt",
            "765748",
            "27732508",
        ),
        (&Day02 {}, "Day02", "inputs/input02.txt", "334", "400"),
        (&day03, "Day03", "inputs/input03.txt", "188741603", "67269798"),
    ];

    for (problem, name, path, part1_solution, part2_solution) in problems {
        let file = std::fs::read_to_string(path).unwrap();

        let part1 = problem.part1(file.lines());
        let part1_correct = part1 == part1_solution;
        println!(
            "{} part 1: {} {}",
            name,
            part1,
            checkmark_or_cross(part1_correct)
        );

        let part2 = problem.part2(file.lines());
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
