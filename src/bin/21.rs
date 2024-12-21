use std::collections::VecDeque;

use hashbrown::HashMap;
use itertools::Itertools;

use cached::proc_macro::cached;
use cached::SizedCache;

#[rustfmt::skip]
#[allow(clippy::type_complexity)]
const NUMPAD: [(u8, [Option<(u8, u8)>; 4]); 11] = [
    (b'0', [Some((b'2', b'^')), Some((b'A', b'>')), None, None]),
    (b'1', [Some((b'2', b'>')), Some((b'4', b'^')), None, None]),
    (b'2', [Some((b'0', b'v')), Some((b'1', b'<')), Some((b'3', b'>')), Some((b'5', b'^')),]), 
    (b'3', [Some((b'2', b'<')), Some((b'6', b'^')), Some((b'A', b'v')), None,]), 
    (b'4', [Some((b'1', b'v')), Some((b'5', b'>')), Some((b'7', b'^')), None,]), 
    (b'5', [Some((b'2', b'v')), Some((b'4', b'<')), Some((b'6', b'>')), Some((b'8', b'^')),]), 
    (b'6', [Some((b'3', b'v')), Some((b'5', b'<')), Some((b'9', b'^')), None,]), 
    (b'7', [Some((b'4', b'v')), Some((b'8', b'>')), None, None]), 
    (b'8', [Some((b'5', b'v')), Some((b'7', b'<')), Some((b'9', b'>')), None,]), 
    (b'9', [Some((b'6', b'v')), Some((b'8', b'<')), None, None]),
    (b'A', [Some((b'0', b'<')), Some((b'3', b'^')), None, None]),
];

#[rustfmt::skip]
#[allow(clippy::type_complexity)]
const KEYPAD: [(u8, [Option<(u8, u8)>; 3]); 5] = [
    (b'^', [Some((b'A', b'>')), Some((b'v', b'v')), None]),
    (b'<', [Some((b'v', b'>')), None, None]),
    (b'v', [Some((b'<', b'<')), Some((b'^', b'^')), Some((b'>', b'>'))]),
    (b'>', [Some((b'v', b'<')), Some((b'A', b'^')), None]),
    (b'A', [Some((b'^', b'<')), Some((b'>', b'v')), None]),
];

#[cached(
    ty = "SizedCache<String, Vec<VecDeque<u8>>>",
    create = "{ SizedCache::with_size(150) }",
    convert = r#"{ format!("{}{s}{e}", graph.len()) }"#
)]
fn bfs(graph: &HashMap<u8, Vec<(u8, u8)>>, s: u8, e: u8) -> Vec<VecDeque<u8>> {
    let mut res = Vec::new();

    let mut queue = VecDeque::new();
    let mut shortest = usize::MAX;

    queue.push_back((s, VecDeque::new()));

    while let Some((n, mut p)) = queue.pop_front() {
        if n == e {
            if shortest == usize::MAX {
                shortest = p.len();
            }

            if p.len() == shortest {
                p.push_back(b'A');
                res.push(p);
            }
            continue;
        }

        if p.len() >= shortest {
            continue;
        }

        for (nn, c) in graph[&n].iter().copied() {
            let mut np = p.clone();
            np.push_back(c);

            queue.push_back((nn, np));
        }
    }

    res
}

#[cached(
    ty = "SizedCache<String, usize>",
    create = "{ SizedCache::with_size(100000) }",
    convert = r#"{ format!("{sequence:?}{level}{graph}") }"#
)]
fn dfs(
    graphs: &[HashMap<u8, Vec<(u8, u8)>>],
    graph: usize,
    mut sequence: VecDeque<u8>,
    level: usize,
) -> usize {
    let graph = &graphs[graph];
    sequence.push_front(b'A');
    let mut res = 0;

    for (n1, n2) in sequence.into_iter().tuple_windows() {
        let paths = bfs(graph, n1, n2);
        if level == 0 {
            res += paths.into_iter().map(|p| p.len()).min().unwrap();
        } else {
            res += paths
                .into_iter()
                .map(|p| dfs(graphs, 1, p, level - 1))
                .min()
                .unwrap();
        }
    }

    res
}

fn build_graphs() -> [HashMap<u8, Vec<(u8, u8)>>; 2] {
    let mut numpad = HashMap::new();
    for (k, v) in NUMPAD.iter() {
        numpad.insert(*k, v.iter().filter_map(|&x| x).collect_vec());
    }

    let mut keypad = HashMap::new();
    for (k, v) in KEYPAD.iter() {
        keypad.insert(*k, v.iter().filter_map(|&x| x).collect_vec());
    }

    [numpad, keypad]
}

fn solve_chain(input: &str, level: usize) -> usize {
    let graphs = build_graphs();

    let mut res = 0;
    for line in input.lines() {
        let n: usize = line.strip_suffix("A").unwrap().parse().unwrap();
        let sequence = VecDeque::from_iter(line.as_bytes().iter().copied());
        let min_len = dfs(&graphs, 0, sequence, level);
        res += min_len * n;
    }

    res
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(solve_chain(input, 2))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(solve_chain(input, 25))
}

aoc::solution!(21);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(&aoc::template::read_file("examples", 21)),
            Some(126384)
        );
    }
    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(&aoc::template::read_file("examples", 21)),
            Some(154115708116294)
        );
    }
}
