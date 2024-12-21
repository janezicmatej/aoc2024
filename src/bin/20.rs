use std::collections::VecDeque;

use aoc::grid_vec::{Direction, Grid, Point};
use hashbrown::HashMap;

fn parse_input(input: &str) -> (Grid, Point, Point) {
    let mut grid: Grid = input.parse().unwrap();
    let start = grid.find(b'S').unwrap();
    let end = grid.find(b'E').unwrap();

    grid[start] = b'.';
    grid[end] = b'.';

    (grid, start, end)
}
fn bfs(
    grid: &Grid,
    start: Point,
    limit: Option<usize>,
    cheat: bool,
    visited: &mut HashMap<Point, usize>,
    queue: &mut VecDeque<(Point, usize)>,
) {
    visited.clear();
    queue.clear();

    queue.push_back((start, 0));

    while let Some((p, l)) = queue.pop_front() {
        if visited.contains_key(&p) {
            continue;
        }
        visited.insert(p, l);

        for d in Direction::CROSS {
            let n = p + d;

            let nv = grid.get(&n);
            let uwnv = *nv.unwrap_or(&b'#');
            let valid_neigh = (cheat && nv.is_some()) || uwnv == b'.';
            let valid_length = l < limit.unwrap_or(usize::MAX);
            let unvisited = !visited.contains_key(&n);

            if valid_neigh && valid_length && unvisited {
                queue.push_back((n, l + 1));
            }
        }
    }
}

fn get_cheat_neighbours_and_benchmark(
    grid: &Grid,
    start: Point,
    cheat_limit: usize,
) -> HashMap<Point, Vec<(Point, usize)>> {
    let mut map = HashMap::<Point, Vec<(Point, usize)>>::new();

    let mut reuse_queue = VecDeque::new();
    let mut reuse_visited = HashMap::new();

    let mut valid_points = HashMap::new();

    bfs(
        grid,
        start,
        None,
        false,
        &mut valid_points,
        &mut reuse_queue,
    );

    for (&s, _) in valid_points.iter() {
        bfs(
            grid,
            s,
            Some(cheat_limit),
            true,
            &mut reuse_visited,
            &mut reuse_queue,
        );
        for (&point, &length) in reuse_visited.iter().filter(|(&p, _)| grid[p] == b'.') {
            map.entry(s).or_default().push((point, length));
        }
    }

    map
}

fn cheated_bfs(
    grid: &Grid,
    start: Point,
    end: Point,
    cheat_limit: usize,
    increase_limit: usize,
) -> usize {
    let neighbours = get_cheat_neighbours_and_benchmark(grid, start, cheat_limit);

    let mut reuse_queue = VecDeque::new();

    let mut from_start = HashMap::new();
    let mut from_end = HashMap::new();

    bfs(grid, start, None, false, &mut from_start, &mut reuse_queue);
    bfs(grid, end, None, false, &mut from_end, &mut reuse_queue);

    let benchmark = from_start[&end];

    let mut count = 0;

    for node in from_start.keys() {
        for (neigh, cheat_dist) in neighbours[node].iter() {
            let start_dist = from_start[node];
            let end_dist = from_end[neigh];

            let dist = start_dist + cheat_dist + end_dist;
            if dist < benchmark && dist.abs_diff(benchmark) >= increase_limit {
                count += 1;
            }
        }
    }

    count
}

pub fn part_one(input: &str) -> Option<usize> {
    let (grid, start, end) = parse_input(input);
    cheated_bfs(&grid, start, end, 2, 100).into()
}

pub fn part_two(input: &str) -> Option<usize> {
    let (grid, start, end) = parse_input(input);
    cheated_bfs(&grid, start, end, 20, 100).into()
}

aoc::solution!(20);
