use std::cmp::Ordering;

use hashbrown::HashSet;
use itertools::Itertools;

#[derive(Debug)]
struct PageUpdates {
    raw: Vec<usize>,
    sorted: Vec<usize>,
}

impl PageUpdates {
    fn middle(&self) -> usize {
        self.sorted[self.sorted.len() / 2]
    }

    fn is_sorted(&self) -> bool {
        self.raw == self.sorted
    }
}

fn parse_updates(input: &str) -> Vec<PageUpdates> {
    let (page_orderings, updates) = input.split_once("\n\n").unwrap();

    let mut ordering = HashSet::new();

    for line in page_orderings.lines() {
        let (a, b) = line.split_once('|').unwrap();
        let a = a.parse().unwrap();
        let b = b.parse().unwrap();
        ordering.insert((a, b));
    }

    let cmp = |&a: &usize, &b: &usize| {
        if ordering.contains(&(a, b)) {
            Ordering::Less
        } else if ordering.contains(&(b, a)) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    };

    let mut page_updates = vec![];
    for update in updates.lines() {
        let raw = update
            .split(',')
            .filter_map(|x| x.parse().ok())
            .collect_vec();

        let mut sorted = raw.clone();
        sorted.sort_by(cmp);

        page_updates.push(PageUpdates { raw, sorted });
    }

    page_updates
}

pub fn part_one(input: &str) -> Option<usize> {
    parse_updates(input)
        .iter()
        .filter(|pu| pu.is_sorted())
        .map(PageUpdates::middle)
        .sum::<usize>()
        .into()
}

pub fn part_two(input: &str) -> Option<usize> {
    parse_updates(input)
        .iter()
        .filter(|pu| !pu.is_sorted())
        .map(PageUpdates::middle)
        .sum::<usize>()
        .into()
}

aoc::solution!(5);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(&aoc::template::read_file("examples", 5)),
            Some(143)
        );
    }
    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(&aoc::template::read_file("examples", 5)),
            Some(123)
        );
    }
}
