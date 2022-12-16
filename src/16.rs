mod utils;

use std::{cmp, collections::HashMap};

fn main() {
    let input = utils::read_input(16);
    let valves = parse(&input);
    let distances = distances(&valves);

    println!("{}", part1(&valves, &distances));
    println!("{}", part2(&valves, &distances));
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Valve<'a> {
    index: usize,
    id: &'a str,
    flow: u32,
    tunnels: Vec<&'a str>,
}

fn parse(input: &str) -> Vec<Valve> {
    input
        .lines()
        .enumerate()
        .map(|(index, line)| {
            let parts: Vec<_> = line
                .split(|c: char| !c.is_ascii_alphanumeric())
                .filter(|s| !s.is_empty())
                .collect();
            Valve {
                index,
                id: parts[1],
                flow: parts[5].parse().unwrap(),
                tunnels: parts[10..].to_vec(),
            }
        })
        .collect()
}

type Distances<'a> = HashMap<(&'a str, &'a str), u32>;

// Floyd–Warshall
fn distances<'a>(valves: &'a [Valve]) -> Distances<'a> {
    let mut distances = HashMap::new();
    for i in valves {
        for j in valves {
            if i.tunnels.contains(&j.id) {
                distances.insert((i.id, j.id), 1);
            }
        }
    }
    for k in valves {
        for i in valves {
            for j in valves {
                let Some(&ik) = distances.get(&(i.id, k.id)) else {
                    continue;
                };
                let Some(&kj) = distances.get(&(k.id, j.id)) else {
                    continue;
                };
                let ij = distances.entry((i.id, j.id)).or_insert(u32::MAX);
                *ij = cmp::min(*ij, ik + kj);
            }
        }
    }
    distances
}

fn walk(valves: &[Valve], distances: &Distances, minutes: u32) -> HashMap<u64, u32> {
    let valves: Vec<_> = valves.iter().filter(|valve| valve.flow > 0).collect();
    let mut paths = HashMap::new();
    walk_inner(&valves, distances, minutes, "AA", 0, 0, &mut paths);
    paths
}

fn walk_inner<'a>(
    valves: &[&Valve],
    distances: &Distances,
    minutes: u32,
    valve: &str,
    opened_valves: u64,
    flow: u32,
    paths: &'a mut HashMap<u64, u32>,
) {
    paths.insert(
        opened_valves,
        cmp::max(paths.get(&opened_valves).copied().unwrap_or_default(), flow),
    );
    for &next_valve in valves {
        let mask = 1 << next_valve.index;
        if opened_valves & mask != 0 {
            continue;
        }
        let Some(&distance) = distances.get(&(valve, next_valve.id)) else {
            continue;
        };
        let Some(next_minutes) = minutes.checked_sub(distance + 1) else {
            continue;
        };
        walk_inner(
            valves,
            distances,
            next_minutes,
            next_valve.id,
            opened_valves | mask,
            flow + (next_minutes * next_valve.flow),
            paths,
        );
    }
}

fn part1(valves: &[Valve], distances: &Distances) -> u32 {
    walk(valves, distances, 30)
        .values()
        .max()
        .copied()
        .unwrap_or_default()
}

fn part2(valves: &[Valve], distances: &Distances) -> u32 {
    let paths = walk(valves, distances, 26);
    let mut flow = 0;
    for (opened_valves1, flow1) in paths.iter() {
        for (opened_valves2, flow2) in paths.iter() {
            if opened_valves1 & opened_valves2 == 0 {
                flow = cmp::max(flow, flow1 + flow2)
            }
        }
    }
    flow
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn test_parse() {
        let valves = parse(EXAMPLE);
        assert_eq!(
            valves[1],
            Valve {
                index: 1,
                id: "BB",
                flow: 13,
                tunnels: vec!["CC", "AA"]
            }
        );
    }

    #[test]
    fn example() {
        let valves = parse(EXAMPLE);
        let distances = distances(&valves);
        assert_eq!(part1(&valves, &distances), 1651);
        assert_eq!(part2(&valves, &distances), 1707);
    }
}
