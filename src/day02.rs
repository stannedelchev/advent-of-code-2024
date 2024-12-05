use crate::problem::ProblemLines;
use itertools::Itertools;
use rayon::prelude::*;
pub struct Day02;

impl ProblemLines for Day02 {
    fn part1(&self, lines: std::str::Lines) -> String {
        lines
            .collect_vec()
            .into_par_iter()
            .map(|s| {
                s.split(' ')
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
            })
            .filter(is_safe)
            .count()
            .to_string()
    }

    fn part2(&self, lines: std::str::Lines) -> String {
        lines
            .collect_vec()
            .into_par_iter()
            .map(|s| {
                s.split(' ')
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
            })
            .filter(|v| {
                is_safe(v)
                    || (0..v.len()).any(move |i| {
                        let mut clone = v.clone();
                        clone.remove(i);
                        is_safe(&clone)
                    })
            })
            .count()
            .to_string()
    }
}

#[inline]
fn is_safe(report: &Vec<i32>) -> bool {
    is_sorted(report) && is_near_adjacent(report)
}

#[inline]
fn is_near_adjacent(report: &Vec<i32>) -> bool {
    let mut is_valid = true;
    for i in 0..report.len() - 1 {
        let diff = report[i].abs_diff(report[i + 1]);
        is_valid = is_valid && (1 <= diff && diff <= 3);
    }

    is_valid
}

#[inline]
fn is_sorted(report: &Vec<i32>) -> bool {
    report.is_sorted() || report.iter().rev().is_sorted()
}
