use std::{
    collections::{HashMap, VecDeque},
    path::Path,
    time::Instant,
};

fn process_p1(path: impl AsRef<Path>, n: usize) -> usize {
    let problem = std::fs::read_to_string(path).unwrap();
    let graph = problem
        .lines()
        .map(|line| {
            let (module, cables) = line.split_once(" -> ").unwrap();
            (module, cables.split(", ").collect::<Vec<_>>())
        })
        .collect::<HashMap<_, _>>();
    let mut switches = graph
        .keys()
        .filter(|m| m.starts_with('%'))
        .map(|m| (&m[1..], false))
        .collect::<HashMap<_, _>>();
    let mut memo = graph.iter().fold(
        HashMap::<&str, HashMap<&str, bool>>::new(),
        |mut memo, (&s, next)| {
            next.iter()
                .filter(|t| graph.contains_key(format!("&{t}").as_str()))
                .for_each(|&t| {
                    if let Some(memo) = memo.get_mut(t) {
                        memo.insert(&s[1..], false);
                    } else {
                        memo.insert(t, HashMap::from([(&s[1..], false)]));
                    };
                });
            memo
        },
    );
    let (mut lcount, mut hcount) = (0, 0);
    for _ in 0..n {
        lcount += 1;
        let mut queue = graph
            .get("broadcaster")
            .unwrap()
            .iter()
            .map(|&m| ("broadcaster", false, m))
            .collect::<VecDeque<_>>();
        while let Some((sm, p, tm)) = queue.pop_front() {
            if p {
                hcount += 1
            } else {
                lcount += 1
            };
            if let Some(cables) = graph.get(format!("%{tm}").as_str()) {
                if !p {
                    let switch = switches.get_mut(tm).unwrap();
                    *switch = !*switch;
                    cables
                        .iter()
                        .for_each(|&m| queue.push_back((tm, *switch, m)))
                }
            }
            if let Some(cables) = graph.get(format!("&{tm}").as_str()) {
                let map = memo.get_mut(tm).unwrap();
                *map.get_mut(sm).unwrap() = p;
                if map.values().all(|p| *p) {
                    cables.iter().for_each(|&m| queue.push_back((tm, false, m)))
                } else {
                    cables.iter().for_each(|&m| queue.push_back((tm, true, m)))
                }
            }
        }
    }
    lcount * hcount
}

#[test]
fn test_process_p1_e1() {
    let path = std::env::temp_dir().join("test_p1.dat");
    std::fs::write(
        &path,
        "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a",
    )
    .unwrap();
    assert_eq!(process_p1(path, 1), 32)
}

#[test]
fn test_process_p1_e2() {
    let path = std::env::temp_dir().join("test_p1.dat");
    std::fs::write(
        &path,
        "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output",
    )
    .unwrap();
    assert_eq!(process_p1(path, 1000), 11687500)
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm_of_iter<I: Iterator<Item = usize>>(numbers: I) -> usize {
    numbers.fold(1, |acc, num| acc * (num / gcd(acc, num)))
}

/// This is adapted from a solution found on redit. I have yet
/// to understand the black magic behind it.
fn process_p2(path: impl AsRef<Path>) -> usize {
    let problem = std::fs::read_to_string(path).unwrap();
    let graph = problem
        .lines()
        .map(|line| {
            let (module, cables) = line.split_once(" -> ").unwrap();
            (module, cables.split(", ").collect::<Vec<_>>())
        })
        .collect::<HashMap<_, _>>();
    let mut res = vec![];
    graph.get("broadcaster").unwrap().iter().for_each(|&m| {
        let mut flipflop = m;
        let mut bin = "".to_string();
        loop {
            let g = graph.get(format!("%{}", flipflop).as_str()).unwrap();
            bin = format!(
                "{}{bin}",
                if g.len() == 2 || !graph.contains_key(format!("%{}", g[0]).as_str()) {
                    1
                } else {
                    0
                }
            );
            let next_flipflops = g
                .iter()
                .filter(|m| graph.contains_key(format!("%{}", m).as_str()))
                .copied()
                .collect::<Vec<_>>();
            if next_flipflops.is_empty() {
                break;
            }
            flipflop = next_flipflops[0];
        }
        res.push(usize::from_str_radix(&bin, 2).unwrap())
    });
    lcm_of_iter(res.into_iter())
}

fn main() {
    let t0 = Instant::now();
    let result_p1 = process_p1("data/day20.txt", 1000);
    let t1 = Instant::now();
    println!("The result of p1 is {}. ({:?})", result_p1, t1 - t0);
    let result_p2 = process_p2("data/day20.txt");
    let t2 = Instant::now();
    println!("The result of p2 is {}. ({:?})", result_p2, t2 - t1);
}
