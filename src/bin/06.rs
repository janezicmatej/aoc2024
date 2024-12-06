use hashbrown::HashSet;
use itertools::Itertools;

const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn count_steps(
    grid: &[Vec<char>],
    start: (usize, usize),
    visited: &mut HashSet<(usize, usize, usize)>,
) -> Option<usize> {
    visited.clear();

    let (mut i, mut j) = start;
    let mut turn = 0;

    loop {
        if !visited.insert((i, j, turn % 4)) {
            return None;
        }
        let (di, dj) = DIRS[turn % 4];
        let (ni, nj) = (i.wrapping_add(di as usize), j.wrapping_add(dj as usize));

        match grid.get(ni).and_then(|row| row.get(nj)) {
            Some(g) => match g {
                '.' => (i, j) = (ni, nj),
                '#' => turn += 1,
                _ => unreachable!(),
            },
            None => break,
        }
    }
    visited.len().into()
}

fn parse_grid(input: &str) -> (Vec<Vec<char>>, (usize, usize)) {
    let mut grid = input.lines().map(|x| x.chars().collect_vec()).collect_vec();

    let (_, i, j) = grid
        .iter()
        .enumerate()
        .flat_map(|(i, x)| x.iter().copied().enumerate().map(move |(j, y)| (y, i, j)))
        .find(|x| x.0 == '^')
        .unwrap();

    grid[i][j] = '.';

    (grid, (i, j))
}

pub fn part_one(input: &str) -> Option<usize> {
    let (grid, start) = parse_grid(input);

    let mut visited = HashSet::new();
    count_steps(&grid, start, &mut visited);

    HashSet::<(usize, usize)>::from_iter(visited.into_iter().map(|(x, y, _)| (x, y)))
        .len()
        .into()
}

pub fn part_two(input: &str) -> Option<usize> {
    let (mut grid, start) = parse_grid(input);

    let mut count = 0;
    let mut visited = HashSet::new();

    for oi in 0..grid.len() {
        for oj in 0..grid[0].len() {
            if (oi, oj) == start || grid[oi][oj] == '#' {
                continue;
            }

            grid[oi][oj] = '#';
            if count_steps(&grid, start, &mut visited).is_none() {
                count += 1;
            }
            grid[oi][oj] = '.';
        }
    }

    count.into()
}

aoc::solution!(6);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&aoc::template::read_file("examples", 6)), Some(41));
    }
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&aoc::template::read_file("examples", 6)), Some(6));
    }
}
