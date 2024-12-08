use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

#[allow(clippy::type_complexity)]
fn parse_input(input: &str) -> (Vec<Vec<char>>, HashMap<char, Vec<(isize, isize)>>) {
    let grid = input.lines().map(|s| s.chars().collect_vec()).collect_vec();

    let mut antennas: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, cell) in row.iter().copied().enumerate().filter(|(_, c)| *c != '.') {
            antennas
                .entry(cell)
                .or_default()
                .push((i as isize, j as isize));
        }
    }

    (grid, antennas)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (grid, antennas) = parse_input(input);

    let mut set = HashSet::new();
    for v in antennas.values() {
        for (s, (i, j)) in v.iter().copied().enumerate() {
            for (ii, jj) in v.iter().copied().skip(s + 1) {
                let (di, dj) = (i - ii, j - jj);

                let (a00, a01) = (i + di, j + dj);
                let (a10, a11) = (ii - di, jj - dj);

                if grid
                    .get(a00 as usize)
                    .and_then(|row| row.get(a01 as usize))
                    .is_some()
                {
                    set.insert((a00, a01));
                }

                if grid
                    .get(a10 as usize)
                    .and_then(|row| row.get(a11 as usize))
                    .is_some()
                {
                    set.insert((a10, a11));
                }
            }
        }
    }

    set.len().into()
}

pub fn part_two(input: &str) -> Option<usize> {
    let (grid, antennas) = parse_input(input);

    let mut set = HashSet::new();
    for v in antennas.values() {
        for (s, (i, j)) in v.iter().copied().enumerate() {
            for (ii, jj) in v.iter().copied().skip(s + 1) {
                let (di, dj) = (i - ii, j - jj);

                let (mut a00, mut a01) = (i, j);
                let (mut a10, mut a11) = (ii, jj);

                while grid
                    .get(a00 as usize)
                    .and_then(|r| r.get(a01 as usize))
                    .is_some()
                {
                    set.insert((a00, a01));
                    (a00, a01) = (a00 + di, a01 + dj);
                }

                while grid
                    .get(a10 as usize)
                    .and_then(|r| r.get(a11 as usize))
                    .is_some()
                {
                    set.insert((a10, a11));
                    (a10, a11) = (a10 - di, a11 - dj);
                }
            }
        }
    }

    set.len().into()
}

aoc::solution!(8);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&aoc::template::read_file("examples", 8)), Some(14));
    }
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&aoc::template::read_file("examples", 8)), Some(34));
    }
}
