use std::collections::BTreeSet;

use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

fn parse_graph(input: &str) -> HashMap<&str, BTreeSet<&str>> {
    let nodes: HashSet<_> = input.lines().filter_map(|x| x.split_once('-')).collect();
    let mut graph: HashMap<_, BTreeSet<_>> = HashMap::new();
    for (n1, n2) in nodes.iter().copied() {
        graph.entry(n1).or_default().insert(n2);
        graph.entry(n2).or_default().insert(n1);
    }

    graph
}

pub fn part_one(input: &str) -> Option<usize> {
    let graph = parse_graph(input);

    let mut triples = HashSet::new();
    for (n1, neigh) in graph.iter() {
        for n2 in neigh.iter().copied() {
            for n3 in graph[n2].iter() {
                if graph[n3].contains(n1) && n1.starts_with('t') {
                    let mut triple = [n1, n2, n3];
                    triple.sort();
                    triples.insert(triple);
                }
            }
        }
    }

    triples.len().into()
}

pub fn part_two(input: &str) -> Option<String> {
    let graph = parse_graph(input);

    let p: BTreeSet<_> = graph.keys().copied().collect();
    let r = HashSet::new();
    let x = BTreeSet::new();

    let mut maximals = HashSet::new();

    let mut stack = Vec::new();
    stack.push((r, p, x));

    while let Some((r, mut p, mut x)) = stack.pop() {
        if p.is_empty() && x.is_empty() {
            let mut password = r.iter().copied().collect_vec();
            password.sort();

            maximals.insert(password.join(","));
        }

        while let Some(pp) = p.pop_last() {
            let mut nr = r.clone();
            nr.insert(pp);

            let np = &p & &graph[pp];
            let nx = &x & &graph[pp];

            stack.push((nr, np, nx));

            x.insert(pp);
        }
    }

    maximals.into_iter().max_by_key(|x| x.len())
}

aoc::solution!(23);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&aoc::template::read_file("examples", 23)), Some(7));
    }
    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(&aoc::template::read_file("examples", 23)),
            Some("co,de,ka,ta".to_string())
        );
    }
}
