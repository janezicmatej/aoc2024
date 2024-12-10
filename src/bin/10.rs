use hashbrown::HashMap;
use itertools::Itertools;

fn score_trailhead(
    acc: &mut HashMap<(usize, usize), usize>,
    grid: &[Vec<usize>],
    start: (usize, usize),
) {
    let (i, j) = start;
    let n = grid[i][j];

    if n == 9 {
        *acc.entry((i, j)).or_default() += 1;
    }

    let dirs = [(-1, 0), (0, -1), (1, 0), (0, 1)];

    for (di, dj) in dirs {
        let (ni, nj) = (i.wrapping_add(di as usize), j.wrapping_add(dj as usize));
        if let Some(nn) = grid.get(ni).and_then(|row| row.get(nj)) {
            if *nn == n + 1 {
                score_trailhead(acc, grid, (ni, nj));
            }
        }
    }
}

fn parse_grid(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|x| {
            x.chars()
                .map(|y| y.to_digit(10).unwrap_or(11) as usize)
                .collect_vec()
        })
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = parse_grid(input);

    let mut score = 0;
    let mut acc = HashMap::new();

    for (i, row) in grid.iter().enumerate() {
        for (j, _) in row.iter().copied().enumerate().filter(|x| x.1 == 0) {
            acc.clear();
            score_trailhead(&mut acc, &grid, (i, j));
            score += acc.len();
        }
    }

    score.into()
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = parse_grid(input);

    let mut acc = HashMap::new();

    for (i, row) in grid.iter().enumerate() {
        for (j, _) in row.iter().copied().enumerate().filter(|x| x.1 == 0) {
            score_trailhead(&mut acc, &grid, (i, j));
        }
    }

    acc.values().sum::<usize>().into()
}

aoc::solution!(10);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(&aoc::template::read_file("examples", 10)),
            Some(36)
        );
    }
    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(&aoc::template::read_file("examples", 10)),
            Some(81)
        );
    }
}
