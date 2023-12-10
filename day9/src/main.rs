use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    time::Instant,
};

fn process_p1(path: impl AsRef<Path>) -> i64 {
    BufReader::new(File::open(path).unwrap())
        .lines()
        .flatten()
        .map(|line| {
            let mut nums = vec![line
                .split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>()];
            while nums.last().unwrap().iter().any(|n| n != &0) {
                nums.push(
                    nums.last()
                        .unwrap()
                        .windows(2)
                        .map(|slice| slice[1] - slice[0])
                        .collect(),
                )
            }
            nums.iter()
                .rev()
                .skip(1)
                .map(|ns| ns.last().unwrap())
                .sum::<i64>()
        })
        .sum()
}

#[test]
fn test_process_p1() {
    let path = std::env::temp_dir().join("test_p1.dat");
    std::fs::write(
        &path,
        "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45",
    )
    .unwrap();
    assert_eq!(process_p1(path), 114)
}

fn process_p2(path: impl AsRef<Path>) -> i64 {
    BufReader::new(File::open(path).unwrap())
        .lines()
        .flatten()
        .map(|line| {
            let mut nums = vec![line
                .split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>()];
            while nums.last().unwrap().iter().any(|n| n != &0) {
                nums.push(
                    nums.last()
                        .unwrap()
                        .windows(2)
                        .map(|slice| slice[1] - slice[0])
                        .collect(),
                )
            }
            nums.iter().rev().skip(1).fold(0, |acc, ns| ns[0] - acc)
        })
        .sum()
}

#[test]
fn test_process_p2() {
    let path = std::env::temp_dir().join("test_p2.dat");
    std::fs::write(
        &path,
        "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45",
    )
    .unwrap();
    assert_eq!(process_p2(path), 2)
}

fn main() {
    let t0 = Instant::now();
    let result_p1 = process_p1("data/day9.txt");
    let t1 = Instant::now();
    let result_p2 = process_p2("data/day9.txt");
    let t2 = Instant::now();
    println!("The result of p1 is {}. ({:?})", result_p1, t1 - t0);
    println!("The result of p2 is {}. ({:?})", result_p2, t2 - t1);
}
