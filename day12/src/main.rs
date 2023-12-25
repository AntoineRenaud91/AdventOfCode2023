use itertools::Itertools;
use std::time::Instant;

#[cfg(test)]
const TEST_CASE: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

fn count_valids(record: &str, counts: &[usize]) -> usize {
    let record = format!(".{}", record.trim_matches('.'));
    *counts
        .iter()
        .fold(
            record.chars().fold(vec![1], |mut acc, c| {
                acc.push(if c != '#' { 1 } else { 0 });
                acc
            }),
            |current, count| {
                record
                    .chars()
                    .enumerate()
                    .fold((vec![0], 0), |(mut next, mut chunk), (i, c)| {
                        if c != '.' {
                            chunk += 1
                        } else {
                            chunk = 0
                        };
                        if c != '#' {
                            next.push(*next.last().unwrap())
                        } else {
                            next.push(0)
                        }
                        if &chunk >= count && record.chars().nth(i - count).unwrap() != '#' {
                            *next.last_mut().unwrap() += current[i - count]
                        }
                        (next, chunk)
                    })
                    .0
            },
        )
        .last()
        .unwrap()
}

#[test]
fn test_count_valid() {
    assert_eq!(count_valids("???.###", &[1, 1, 3]), 1);
    assert_eq!(count_valids(".??..??...?##.", &[1, 1, 3]), 4);
    assert_eq!(count_valids("?#?#?#?#?#?#?#?", &[1, 3, 1, 6]), 1);
    assert_eq!(count_valids("????.######..#####.", &[1, 6, 5]), 4);
    assert_eq!(count_valids("?###????????", &[3, 2, 1]), 10);
}

fn process_p1(data: &str) -> usize {
    data.lines()
        .map(|l| {
            let (record, counts) = l.split_once(' ').unwrap();
            let record = record.trim_end_matches('.');
            let counts = counts
                .split(',')
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            count_valids(record, &counts)
        })
        .sum()
}

#[test]
fn test_process_p1() {
    assert_eq!(process_p1(TEST_CASE), 21)
}

fn process_p2(data: &str) -> usize {
    data.lines()
        .map(|l| {
            let (record, counts) = l.split_once(' ').unwrap();
            let counts = counts
                .split(',')
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            let record = std::iter::repeat(record)
                .take(5)
                .collect::<Vec<&str>>()
                .join("?");
            let counts = counts
                .iter()
                .cycle()
                .take(counts.len() * 5)
                .cloned()
                .collect_vec();
            count_valids(&record, &counts)
        })
        .sum()
}

#[test]
fn test_process_p2() {
    assert_eq!(process_p2(TEST_CASE), 525152)
}

fn main() {
    let data = std::fs::read_to_string("data/day12.txt").unwrap();
    let t0 = Instant::now();
    let result_p1 = process_p1(&data);
    let t1 = Instant::now();
    println!("The result of p1 is {}. ({:?})", result_p1, t1 - t0);
    let result_p2 = process_p2(&data);
    let t2 = Instant::now();
    println!("The result of p2 is {}. ({:?})", result_p2, t2 - t1);
}
