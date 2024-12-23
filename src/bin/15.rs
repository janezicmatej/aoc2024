use hashbrown::HashSet;
use itertools::Itertools;

struct ParsedInput {
    map: Vec<Vec<char>>,
    dirs: Vec<(isize, isize)>,
    start: (usize, usize),
}

fn dir_mapper(c: char) -> (isize, isize) {
    match c {
        '>' => (0, 1),
        '<' => (0, -1),
        'v' => (1, 0),
        '^' => (-1, 0),
        _ => unreachable!("found dir {c}"),
    }
}

fn parse_input(input: &str, extend: bool) -> ParsedInput {
    let (umap, uins) = input.split_once("\n\n").unwrap();

    let mut map = Vec::new();

    for line in umap.lines() {
        let mut row: Vec<char> = line.chars().collect();
        if extend {
            row = line
                .chars()
                .flat_map(|x| {
                    let y = match x {
                        '#' => "##",
                        'O' => "[]",
                        '.' => "..",
                        '@' => "@.",
                        _ => unreachable!("found {x}"),
                    };
                    y.chars()
                })
                .collect()
        };

        map.push(row);
    }

    let dirs = uins
        .lines()
        .flat_map(|x| x.chars().map(dir_mapper))
        .collect_vec();

    let (si, sj, _) = map
        .iter()
        .enumerate()
        .flat_map(|(i, x)| x.iter().copied().enumerate().map(move |(j, y)| (i, j, y)))
        .find(|x| x.2 == '@')
        .unwrap();

    ParsedInput {
        map,
        dirs,
        start: (si, sj),
    }
}

fn boxes_gps(map: &[Vec<char>]) -> usize {
    map.iter()
        .enumerate()
        .flat_map(|(i, x)| {
            x.iter()
                .enumerate()
                .filter(|(_, y)| ['O', '['].contains(y))
                .map(move |(j, _)| 100 * i + j)
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<usize> {
    let ParsedInput {
        mut map,
        dirs,
        start: (mut si, mut sj),
    } = parse_input(input, false);

    for (di, dj) in dirs.iter().copied() {
        let mut box_count = 0;

        loop {
            if map[si][sj] == 'O' {
                box_count += 1;
                map[si][sj] = '.';
            }

            map[si][sj] = '.';

            let ni = si.wrapping_add(di as usize);
            let nj = sj.wrapping_add(dj as usize);

            if map[ni][nj] == '#' {
                break;
            }

            (si, sj) = (ni, nj);

            if map[si][sj] == '.' {
                break;
            }
        }

        let (di, dj) = (-di, -dj);

        for _ in 0..box_count {
            map[si][sj] = 'O';
            si = si.wrapping_add(di as usize);
            sj = sj.wrapping_add(dj as usize);
        }

        map[si][sj] = '@';
    }

    boxes_gps(&map).into()
}

fn can_move_and_mark(
    acc: &mut HashSet<(usize, usize, usize)>,
    mark: usize,
    map: &[Vec<char>],
    loc: (usize, usize),
    dir: (isize, isize),
) -> bool {
    let (li, mut lj) = loc;

    match map[li][lj] {
        '.' => return true,
        '#' => return false,
        ']' => lj -= 1,
        _ => (),
    }

    let mut cmm = |l, dir| can_move_and_mark(acc, mark + 1, map, l, dir);

    let can_move = match dir {
        (0, -1) => cmm((li, lj - 1), dir),
        (0, 1) => cmm((li, lj + 2), dir),
        (-1, 0) => cmm((li - 1, lj), dir) && cmm((li - 1, lj + 1), dir),
        (1, 0) => cmm((li + 1, lj), dir) && cmm((li + 1, lj + 1), dir),
        x => unreachable!("found dir {x:?}"),
    };

    if can_move {
        acc.insert((li, lj, mark));
    }

    can_move
}

pub fn part_two(input: &str) -> Option<usize> {
    let ParsedInput {
        mut map,
        dirs,
        start: (mut si, mut sj),
    } = parse_input(input, true);

    let mut marker = HashSet::new();
    let mut sorter = Vec::new();

    for (di, dj) in dirs.iter().copied() {
        marker.clear();
        sorter.clear();

        let ni = si.wrapping_add(di as usize);
        let nj = sj.wrapping_add(dj as usize);

        if can_move_and_mark(&mut marker, 0, &map, (ni, nj), (di, dj)) {
            sorter.extend(marker.drain());
            sorter.sort_by_key(|x| -(x.2 as isize));

            for (mi, mj, _) in sorter.iter().copied() {
                let nmi = mi.wrapping_add(di as usize);
                let nmj = mj.wrapping_add(dj as usize);

                map[mi][mj] = '.';
                map[mi][mj + 1] = '.';
                map[nmi][nmj] = '[';
                map[nmi][nmj + 1] = ']';
            }

            map[si][sj] = '.';
            (si, sj) = (ni, nj);
            map[si][sj] = '@';
        }
    }

    boxes_gps(&map).into()
}

aoc::solution!(15);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(&aoc::template::read_file("examples", 15)),
            Some(10092)
        );
    }
    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(&aoc::template::read_file("examples", 15)),
            Some(9021)
        );
    }
}
