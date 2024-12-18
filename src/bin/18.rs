use std::collections::VecDeque;

use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

use aoc::{
    grid_vec::{Direction, Point},
    pnt,
};

fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|x| {
            x.split(',')
                .filter_map(|y| y.parse().ok())
                .collect_tuple()
                .unwrap()
        })
        .map(|x: (usize, usize)| Point { i: x.1, j: x.0 })
        .collect_vec()
}

fn bfs(
    visited: &mut HashMap<Point, usize>,
    queue: &mut VecDeque<(Point, usize)>,
    corruption: &HashSet<Point>,
    end: Point,
) -> Option<usize> {
    visited.clear();
    queue.clear();

    queue.push_back((pnt!(0, 0), 0));

    while let Some((p, l)) = queue.pop_front() {
        if visited.contains_key(&p) {
            continue;
        }
        visited.insert(p, l);

        if p == end {
            break;
        }

        for d in Direction::CROSS {
            let n = p + d;

            let in_bounds = n.bounded_by(&end);
            let not_visited = !visited.contains_key(&n);
            let not_corrupted = !corruption.contains(&n);

            if in_bounds && not_visited && not_corrupted {
                queue.push_back((n, l + 1));
            }
        }
    }

    visited.get(&end).copied()
}

pub fn part_one(input: &str) -> Option<usize> {
    let falling_bytes = parse_input(input);

    let mut visited = HashMap::new();
    let mut queue = VecDeque::new();

    let end = pnt!(70, 70);
    let corruption = HashSet::from_iter(falling_bytes.iter().copied().take(1024));

    bfs(&mut visited, &mut queue, &corruption, end)
}

pub fn part_two(input: &str) -> Option<String> {
    let falling_bytes = parse_input(input);

    let mut visited = HashMap::new();
    let mut queue = VecDeque::new();

    let end = pnt!(70, 70);
    let mut corruption = HashSet::new();
    let mut c = 0;

    while bfs(&mut visited, &mut queue, &corruption, end).is_some() {
        corruption.insert(falling_bytes[c]);
        c += 1;
    }

    let fb = falling_bytes[c - 1];

    Some(format!("{},{}", fb.j, fb.i))
}

aoc::solution!(18);

