use crate::problem::Problem;

pub struct Day01;

impl Problem for Day01 {
    fn part1(&self, lines: std::str::Lines) -> String {
        let mut left = vec![];
        let mut right = vec![];

        for value in lines.map(parse) {
            left.push(value.0);
            right.push(value.1);
        }
        left.sort();
        right.sort();

        let mut sum = 0;
        for i in 0..left.len() {
            sum += (left[i] - right[i]).abs()
        }

        sum.to_string()
    }

    fn part2(&self, lines: std::str::Lines) -> String {
        let mut left = vec![];
        let mut right = vec![];

        for value in lines.map(parse) {
            left.push(value.0);
            right.push(value.1);
        }

        left.sort();
        right.sort();

        let mut sum = 0;
        for value in left.into_iter() {
            sum += value * right.iter().filter(|v| **v == value).count() as i32;
        }

        sum.to_string()
    }
}

fn parse(s: &str) -> (i32, i32) {
    let left = s[0..5].parse::<i32>().unwrap();
    let right = s[8..].parse::<i32>().unwrap();
    (left, right)
}
