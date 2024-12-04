use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let g = input.lines().map(|x| x.chars().collect_vec()).collect_vec();

    #[rustfmt::skip]
    let dirs = [
        (1, 0), (1, 1), (0, 1), (-1, 1),
        (-1, 0), (-1, -1), (0, -1), (1, -1),
    ];
    let xmas = ['X', 'M', 'A', 'S'];
    let mut count = 0;

    for (i, row) in g.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            for &(di, dj) in &dirs {
                let found_xmas = (0..4).all(|x| {
                    let ni = i.wrapping_add((di * x) as usize);
                    let nj = j.wrapping_add((dj * x) as usize);

                    g.get(ni)
                        .and_then(|y| y.get(nj))
                        .copied()
                        .unwrap_or_default()
                        == xmas[x as usize]
                });

                if found_xmas {
                    count += 1;
                }
            }
        }
    }

    count.into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let g = input.lines().map(|x| x.chars().collect_vec()).collect_vec();

    let dirs = [[(-1, -1), (0, 0), (1, 1)], [(1, -1), (0, 0), (-1, 1)]];
    let x_mas = [['M', 'A', 'S'], ['S', 'A', 'M']];
    let mut count = 0;

    for (i, row) in g.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            let found_x_mas = dirs.iter().all(|d| {
                let s = d.iter().map(|&(di, dj)| {
                    let ni = i.wrapping_add(di as usize);
                    let nj = j.wrapping_add(dj as usize);

                    g.get(ni)
                        .and_then(|y| y.get(nj))
                        .copied()
                        .unwrap_or_default()
                });

                x_mas
                    .iter()
                    .any(|x| x.iter().copied().zip(s.clone()).all(|(a, b)| a == b))
            });

            if found_x_mas {
                count += 1
            }
        }
    }

    count.into()
}

aoc::solution!(4);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&aoc::template::read_file("examples", 4)), Some(18));
    }
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&aoc::template::read_file("examples", 4)), Some(9));
    }
}
