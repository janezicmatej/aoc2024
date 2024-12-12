use hashbrown::HashSet;
use itertools::Itertools;

fn count_sides(perim: &mut [(isize, isize)]) -> usize {
    perim.sort();

    let mut sides = 0;

    let mut k = 0;
    while k < perim.len() {
        let (mut i, mut j) = perim[k];

        while let Some((ni, nj)) = perim.get(k + 1).copied() {
            if i != ni || nj - j != 3 {
                break;
            }

            (i, j) = (ni, nj);
            k += 1;
        }

        sides += 1;
        k += 1;
    }

    sides
}

pub fn solve(input: &str) -> (usize, usize) {
    let grid = input.lines().map(|x| x.chars().collect_vec()).collect_vec();

    let mut price = 0;
    let mut discounted = 0;

    let mut stack = Vec::new();
    let mut visited = HashSet::new();

    let mut ver_sides = Vec::new();
    let mut hor_sides = Vec::new();

    for p in (0..grid.len()).flat_map(|x| (0..grid[x].len()).map(move |y| (x, y))) {
        if visited.contains(&p) {
            continue;
        }

        let matcher = grid[p.0][p.1];

        let mut area = 0;
        ver_sides.clear();
        hor_sides.clear();

        stack.push(p);
        while let Some((i, j)) = stack.pop() {
            if !visited.insert((i, j)) {
                continue;
            }
            area += 1;

            for (di, dj) in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
                let ni = i.wrapping_add(di as usize);
                let nj = j.wrapping_add(dj as usize);

                let fi = i as isize * 3 + di;
                let fj = j as isize * 3 + dj;

                match grid.get(ni).and_then(|row| row.get(nj)) {
                    Some(m) if *m == matcher => stack.push((ni, nj)),
                    _ if dj == 0 => hor_sides.push((fi, fj)),
                    _ if dj != 0 => ver_sides.push((fj, fi)),
                    _ => unreachable!(),
                }
            }
        }

        price += area * (hor_sides.len() + ver_sides.len());
        discounted += area * (count_sides(&mut hor_sides) + count_sides(&mut ver_sides));
    }

    (price, discounted)
}

pub fn part_one(input: &str) -> Option<usize> {
    solve(input).0.into()
}

pub fn part_two(input: &str) -> Option<usize> {
    solve(input).1.into()
}

aoc::solution!(12);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(&aoc::template::read_file("examples", 12)),
            Some(1930)
        );
    }
    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(&aoc::template::read_file("examples", 12)),
            Some(1206)
        );
    }
}
