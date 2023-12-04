use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn process_p1(path: impl AsRef<Path>) -> usize {
    BufReader::new(File::open(path).unwrap())
        .lines()
        .flatten()
        .fold(0usize, |sum, line| {
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
    let path = std::env::temp_dir().join("test_p1.dat");
    std::fs::write(
        &path,
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
    )
    .unwrap();
    assert_eq!(process_p1(path), 13)
}

fn process_p2(path: impl AsRef<Path>) -> usize {
    let nlines = BufReader::new(File::open(&path).unwrap())
        .lines()
        .flatten()
        .count();
    let mut copies = vec![0usize; nlines];
    let mut result = 0usize;
    for (i, line) in BufReader::new(File::open(path).unwrap())
        .lines()
        .flatten()
        .enumerate()
    {
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
    let path = std::env::temp_dir().join("test_p2.dat");
    std::fs::write(
        &path,
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
    )
    .unwrap();
    assert_eq!(process_p2(path), 30)
}

fn main() {
    println!("The result of p1 is {}.", process_p1("data/day4.txt"));
    println!("The result of p2 is {}.", process_p2("data/day4.txt"));
}
