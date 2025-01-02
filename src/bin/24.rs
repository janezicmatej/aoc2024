use std::collections::VecDeque;

use hashbrown::HashMap;
use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Clone, Copy)]
enum Gate {
    And,
    Or,
    XOr,
}

#[derive(Debug)]
struct LogicGate<'a> {
    input_one: &'a str,
    input_two: &'a str,
    output: &'a str,
    gate: Gate,
}

fn parse_input(input: &str) -> (HashMap<&str, bool>, Vec<LogicGate>) {
    let (uvals, uevals) = input.split_once("\n\n").unwrap();
    let vals: HashMap<_, _> = uvals
        .lines()
        .map(|x| x.split_once(": ").unwrap())
        .map(|(k, v)| (k, v == "1"))
        .collect();

    let mut logic_gates = Vec::new();
    let re = Regex::new(r#"((?:\w|\d){3}) (AND|XOR|OR) ((?:\w|\d){3}) -> ((?:\w|\d){3})"#).unwrap();
    for capt in re.captures_iter(uevals) {
        let input_one = capt.get(1).unwrap().as_str();
        let gate = match capt.get(2).unwrap().as_str() {
            "AND" => Gate::And,
            "OR" => Gate::Or,
            "XOR" => Gate::XOr,
            _ => unreachable!(),
        };
        let input_two = capt.get(3).unwrap().as_str();
        let output = capt.get(4).unwrap().as_str();
        logic_gates.push(LogicGate {
            input_one,
            input_two,
            output,
            gate,
        })
    }

    (vals, logic_gates)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (mut vals, logic_gates) = parse_input(input);
    let mut queue = VecDeque::from(logic_gates);

    while let Some(s) = queue.pop_front() {
        if !vals.contains_key(s.input_one) || !vals.contains_key(s.input_two) {
            queue.push_back(s);
            continue;
        }

        let v1 = vals[s.input_one];
        let v2 = vals[s.input_two];

        let o = match s.gate {
            Gate::And => v1 & v2,
            Gate::Or => v1 | v2,
            Gate::XOr => v1 ^ v2,
        };

        vals.insert(s.output, o);
    }

    let mut zs = vals
        .into_iter()
        .filter(|x| x.0.starts_with('z'))
        .collect_vec();
    zs.sort();
    zs.into_iter()
        .enumerate()
        .filter(|x| x.1 .1)
        .map(|(i, _)| 1 << i)
        .sum::<usize>()
        .into()
}

pub fn part_two(_input: &str) -> Option<String> {
    // Used graphviz for this part
    // ```rs
    // let (_, logic_gates) = parse_input(input);
    // println!("digraph A {{");
    // for (idx, lg) in logic_gates.iter().enumerate() {
    //     let LogicGate {
    //         input_one: i1,
    //         input_two: i2,
    //         output: o1,
    //         gate: g,
    //     } = lg;
    //     println!("{i1} -> {g:?}_{idx}");
    //     println!("{i2} -> {g:?}_{idx}");
    //     println!("{g:?}_{idx} -> {o1}");
    // }
    // println!("}}");
    // ```
    //
    // and then pipe to dot
    // ```bash
    // ... | dot -Tsvg -Kneato > grpah.svg
    // ```
    //
    // i looked for errors in the pattern and found below
    // solution for my input

    let mut res = ["fkp", "z06", "z11", "ngr", "z31", "mfm", "bpt", "krj"];
    res.sort();
    res.join(",").to_string().into()
}

aoc::solution!(24);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(&aoc::template::read_file("examples", 24)),
            Some(2024)
        );
    }
}
