use std::{collections::HashMap, time::Instant};

#[cfg(test)]
const TEST_CASE_1: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

#[cfg(test)]
const TEST_CASE_2: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

fn get_graph<'a, L: Iterator<Item = &'a str>>(lines: L) -> HashMap<&'a str, (&'a str, &'a str)> {
    lines
        .map(|line| {
            let source = &line[..3];
            let target_left = &line[7..10];
            let target_right = &line[12..15];
            (source, (target_left, target_right))
        })
        .collect()
}

fn count_to_exit_p1(
    curnode: &str,
    count: usize,
    graph: &HashMap<&str, (&str, &str)>,
    mut instructions: impl Iterator<Item = char>,
) -> usize {
    if curnode == "ZZZ" {
        return count;
    }
    let targets = graph.get(curnode).unwrap();
    match instructions.next().unwrap() {
        'R' => count_to_exit_p1(targets.1, count + 1, graph, instructions),
        'L' => count_to_exit_p1(targets.0, count + 1, graph, instructions),
        _ => panic!("not supposed to append"),
    }
}

fn process_p1(data: &str) -> usize {
    let mut lines = data.lines();
    let instructions = lines.next().unwrap();
    let graph = get_graph(lines.skip(1));
    let (count, curnode) = (0, "AAA");
    count_to_exit_p1(curnode, count, &graph, instructions.chars().cycle())
}

#[test]
fn test_process_p1() {
    assert_eq!(process_p1(TEST_CASE_1), 2)
}

fn count_to_exit_p2(
    curnode: &str,
    count: usize,
    graph: &HashMap<&str, (&str, &str)>,
    mut instructions: impl Iterator<Item = char>,
) -> usize {
    if curnode.ends_with('Z') {
        return count;
    }
    let targets = graph.get(curnode).unwrap();
    match instructions.next().unwrap() {
        'R' => count_to_exit_p2(targets.1, count + 1, graph, instructions),
        'L' => count_to_exit_p2(targets.0, count + 1, graph, instructions),
        _ => panic!("not supposed to append"),
    }
}

fn process_p2(data: &str) -> usize {
    let mut lines = data.lines();
    let instructions = lines.next().unwrap();
    let graph = get_graph(lines.skip(1));
    graph
        .keys()
        .filter(|n| n.ends_with('A'))
        .map(|n| count_to_exit_p2(n, 0, &graph, instructions.chars().cycle()))
        .fold(1, num::integer::lcm)
}

#[test]
fn test_process_p2() {
    assert_eq!(process_p2(TEST_CASE_2), 6)
}

fn main() {
    let data = std::fs::read_to_string("data/day8.txt").unwrap();
    let t0 = Instant::now();
    let result_p1 = process_p1(&data);
    let t1 = Instant::now();
    println!("The result of p1 is {}. ({:?})", result_p1, t1 - t0);
    let result_p2 = process_p2(&data);
    let t2 = Instant::now();
    println!("The result of p2 is {}. ({:?})", result_p2, t2 - t1);
}
