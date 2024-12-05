use crate::problem::ProblemLines;
use regex::Regex;

pub struct Day03 {
    regex_pt1: Regex,
    regex_pt2: Regex,
}

impl Day03 {
    pub fn new() -> Self {
        Day03 {
            regex_pt1: Regex::new(r"mul\((?<first>\d{1,3}),(?<second>\d{1,3})\)").unwrap(),
            regex_pt2: Regex::new(
                r"(don't\(\)|do\(\)|mul\((?<first>\d{1,3}),(?<second>\d{1,3})\))",
            )
            .unwrap(),
        }
    }
}

impl ProblemLines for Day03 {
    fn part1(&self, lines: std::str::Lines) -> String {
        lines
            .flat_map(|line| {
                let captures = self.regex_pt1.captures_iter(line);
                captures.map(|c| {
                    let first = c.get(1).unwrap().as_str().parse::<u64>().unwrap();
                    let second = c.get(2).unwrap().as_str().parse::<u64>().unwrap();
                    first * second
                })
            })
            .sum::<u64>()
            .to_string()
    }

    fn part2(&self, lines: std::str::Lines) -> String {
        let mut enable_mul = 1;
        let mut sum = 0;

        for line in lines {
            let captures = self.regex_pt2.captures_iter(line);
            for c in captures {
                let capture = c.get(0).unwrap();
                match capture.as_str() {
                    "do()" => {
                        enable_mul = 1;
                    }
                    "don't()" => {
                        enable_mul = 0;
                    }
                    _ => {
                        let first_group = c.get(2).unwrap();
                        let second_group = c.get(3).unwrap();
                        let first = first_group.as_str().parse::<u64>().unwrap();
                        let second = second_group.as_str().parse::<u64>().unwrap();
                        sum += first * second * enable_mul
                    }
                }
            }
        }
        sum.to_string()
        //
        // lines
        //     .flat_map(|line| {
        //         self.regex_pt2.captures_iter(line).map(|c| {
        //             match capture.as_str() {
        //                 "do()" => {
        //                     enable_mul = 1;
        //                     0
        //                 }
        //                 "don't()" => {
        //                     enable_mul = 0;
        //                     0
        //                 }
        //                 _ => {
        //                     let first_group = c.get(2).unwrap();
        //                     let second_group = c.get(3).unwrap();
        //                     let first = first_group.as_str().parse::<u64>().unwrap();
        //                     let second = second_group.as_str().parse::<u64>().unwrap();
        //                     first * second * enable_mul
        //                 }
        //             }
        //         })
        //     })
        //     .sum::<u64>()
        //     .to_string()
    }
}
