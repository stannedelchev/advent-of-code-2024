use crate::problem::ProblemLines;
use std::collections::{HashMap, HashSet};

pub struct Day01;

const ESTIMATED_ELEMENTS: usize = 1000;

impl ProblemLines for Day01 {
    fn part1(&self, lines: std::str::Lines) -> String {
        let mut left = Vec::with_capacity(ESTIMATED_ELEMENTS);
        let mut right = Vec::with_capacity(ESTIMATED_ELEMENTS);

        for value in lines.map(parse) {
            left.push(value.0);
            right.push(value.1);
        }
        left.sort();
        right.sort();

        let mut sum = 0;
        for i in 0..left.len() {
            sum += left[i].abs_diff(right[i]);
        }

        sum.to_string()
    }

    fn part2(&self, lines: std::str::Lines) -> String {
        let mut left = HashSet::with_capacity(ESTIMATED_ELEMENTS);
        let mut counts = HashMap::with_capacity(ESTIMATED_ELEMENTS);

        for value in lines.map(parse) {
            left.insert(value.0);
            counts.entry(value.1).and_modify(|v| *v += 1).or_insert(1);
        }

        let mut sum = 0;
        for value in left.iter() {
            sum += value * counts.get(value).unwrap_or(&0);
        }

        sum.to_string()
    }
}

fn parse(s: &str) -> (i32, i32) {
    let left = s[0..5].parse::<i32>().unwrap();
    let right = s[8..].parse::<i32>().unwrap();
    (left, right)
}
