use crate::problem::ProblemLines;
use itertools::Itertools;
use std::cmp::Ordering;
use std::str::Lines;

pub struct Day05 {}

impl ProblemLines for Day05 {
    fn part1(&self, lines: Lines) -> String {
        let (rules, updates) = parse(lines);

        updates
            .into_iter()
            .map(|update| {
                if update.is_sorted_by(|a, b| rules[*a].contains(b)) {
                    update[update.len() / 2]
                } else {
                    0
                }
            })
            .sum::<usize>()
            .to_string()
    }

    fn part2(&self, lines: Lines) -> String {
        let (rules, updates) = parse(lines);
        updates
            .into_iter()
            .filter(|update| !update.is_sorted_by(|a, b| rules[*a].contains(b)))
            .map(|mut update| {
                update.sort_by(|a, b| {
                    if rules[*a].contains(b) {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                });
                update[update.len() / 2]
            })
            .sum::<usize>()
            .to_string()
    }
}

fn parse(lines: Lines) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let mut rules: Vec<Vec<usize>> = Vec::with_capacity(100);
    rules.resize_with(100, Default::default);
    let rules = rules;
    let (rules, updates) = lines.fold((rules, vec![]), |mut acc, v| {
        match v.len() {
            5 => acc.0[parse_fast(&v[0..2])].push(parse_fast(&v[3..])),
            0 => {}
            _ => acc
                .1
                .push(v.split(',').map(|s| parse_fast(s)).collect_vec()),
        }

        acc
    });
    (rules, updates)
}

fn parse_fast(s: &str) -> usize {
    let bytes = s.as_bytes();
    ((bytes[0] - 48) * 10 + bytes[1] - 48) as usize
}
