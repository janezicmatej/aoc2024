use std::{cmp::Reverse, collections::BinaryHeap};

use aoc::grid_vec::{Direction, Grid, Point};
use hashbrown::{HashMap, HashSet};

fn parse_input(input: &str) -> (Grid, Point, Point) {
    let mut grid: Grid = input.parse().unwrap();
    let start = grid.find(b'S').unwrap();
    let end = grid.find(b'E').unwrap();

    grid[start] = b'.';
    grid[end] = b'.';

    (grid, start, end)
}

type Prev = HashMap<(Point, Direction), HashSet<(Point, Direction)>>;
type Visited = HashMap<(Point, Direction), usize>;

fn dijkstra(grid: &Grid, start: Point) -> (Visited, Prev) {
    let mut prev: Prev = HashMap::new();
    let mut visited: Visited = HashMap::new();

    let mut bh = BinaryHeap::new();
    bh.push(Reverse((0, start, Direction::E, start, Direction::E)));

    while let Some(Reverse((len, loc, dir, ploc, pdir))) = bh.pop() {
        let best = visited.entry((loc, dir)).or_insert(usize::MAX);

        if len > *best {
            continue;
        }

        prev.entry((loc, dir)).or_default().insert((ploc, pdir));

        *best = len;

        let n = loc + dir;

        if grid.get(&n).filter(|&&c| c == b'.').is_some() && !visited.contains_key(&(n, dir)) {
            bh.push(Reverse((len + 1, n, dir, loc, dir)));
        }

        let rl = dir.rotate_left();
        if !visited.contains_key(&(loc, rl)) {
            bh.push(Reverse((len + 1000, loc, rl, loc, dir)));
        }

        let rr = dir.rotate_right();
        if !visited.contains_key(&(loc, rr)) {
            bh.push(Reverse((len + 1000, loc, rr, loc, dir)));
        }
    }

    (visited, prev)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (grid, start, end) = parse_input(input);
    dijkstra(&grid, start)
        .0
        .into_iter()
        .filter(|(k, _)| k.0 == end)
        .map(|(_, v)| v)
        .min()
}

pub fn part_two(input: &str) -> Option<usize> {
    let (grid, start, end) = parse_input(input);

    let (visited, prev) = dijkstra(&grid, start);

    let mut stack = Vec::new();
    stack.push(
        prev.keys()
            .filter(|&&k| k.0 == end)
            .min_by_key(|&k| visited.get(k))
            .copied()
            .unwrap(),
    );

    let mut visited = HashSet::new();
    let mut seats = HashSet::new();

    while let Some((loc, dir)) = stack.pop() {
        if !visited.insert((loc, dir)) {
            continue;
        }

        if let Some(prevs) = prev.get(&(loc, dir)) {
            stack.extend(prevs);
        }

        seats.insert(loc);
    }

    seats.len().into()
}

aoc::solution!(16);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(&aoc::template::read_file("examples", 16)),
            Some(11048)
        );
    }
    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(&aoc::template::read_file("examples", 16)),
            Some(64)
        );
    }
}
