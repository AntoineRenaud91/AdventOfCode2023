use std::{collections::HashSet, time::Instant};

#[cfg(test)]
const TEST_CASE: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

fn process_p1(data: &str) -> usize {
    data.lines().fold(0usize, |sum, line| {
        let (win_nums, scratched_nums) = line
            .split_once(':')
            .expect("':' not  in line")
            .1
            .split_once('|')
            .expect("'|' is missing");
        let win_nums = win_nums
            .split(' ')
            .filter_map(|s| {
                if s.is_empty() {
                    return None;
                }
                Some(s.parse::<usize>().expect("not a valid number"))
            })
            .collect::<HashSet<_>>();
        let wins = scratched_nums
            .split(' ')
            .filter_map(|s| {
                if s.is_empty() {
                    return None;
                }
                Some(s.parse::<usize>().expect("not a valid number"))
            })
            .filter(|num| win_nums.contains(num))
            .count();
        if wins == 0 {
            sum
        } else {
            sum + 2usize.pow((wins - 1) as u32)
        }
    })
}

#[test]
fn test_process_p1() {
    assert_eq!(process_p1(TEST_CASE), 13)
}

fn process_p2(data: &str) -> usize {
    let nlines = data.lines().count();
    let mut copies = vec![0usize; nlines];
    let mut result = 0usize;
    for (i, line) in data.lines().enumerate() {
        result += copies[i] + 1;
        let (win_nums, scratched_nums) = line
            .split_once(':')
            .expect("':' not  in line")
            .1
            .split_once('|')
            .expect("'|' is missing");
        let win_nums = win_nums
            .split(' ')
            .filter_map(|s| {
                if s.is_empty() {
                    return None;
                }
                Some(s.parse::<usize>().expect("not a valid number"))
            })
            .collect::<HashSet<_>>();
        let wins = scratched_nums
            .split(' ')
            .filter_map(|s| {
                if s.is_empty() {
                    return None;
                }
                Some(s.parse::<usize>().expect("not a valid number"))
            })
            .filter(|num| win_nums.contains(num))
            .count();
        for j in (i + 1)..(i + 1 + wins) {
            copies[j] += copies[i] + 1
        }
    }
    result
}

#[test]
fn test_process_p2() {
    assert_eq!(process_p2(TEST_CASE), 30)
}

fn main() {
    let data = std::fs::read_to_string("data/day4.txt").unwrap();
    let t0 = Instant::now();
    let result_p1 = process_p1(&data);
    let t1 = Instant::now();
    println!("The result of p1 is {}. ({:?})", result_p1, t1 - t0);
    let result_p2 = process_p2(&data);
    let t2 = Instant::now();
    println!("The result of p2 is {}. ({:?})", result_p2, t2 - t1);
}
