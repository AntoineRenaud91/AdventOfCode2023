use std::{cmp::Ordering, collections::HashMap, fs::read_to_string, path::Path, time::Instant};

const CARDS_P1: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

fn count_occurences_p1(hand: &str) -> HashMap<char, usize> {
    let mut counts = HashMap::from_iter(CARDS_P1.iter().map(|c| (*c, 0)));
    hand.chars().for_each(|c| {
        *counts.get_mut(&c).unwrap() += 1;
    });
    counts
}

fn val_from_counts(counts: &HashMap<char, usize>) -> usize {
    counts
        .values()
        .filter(|&&v| v != 0)
        .map(|&v| 10usize.pow(v as u32 - 1))
        .sum()
}

fn process_p1(path: impl AsRef<Path>) -> usize {
    let map_val: HashMap<char, usize> =
        HashMap::from_iter(CARDS_P1.iter().enumerate().map(|(i, c)| (*c, i + 1)));
    let input = read_to_string(path).unwrap();
    let mut hands_and_bids = input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            (hand, bid.parse::<usize>().unwrap())
        })
        .collect::<Vec<_>>();
    hands_and_bids.sort_by(|(h1, _), (h2, _)| {
        let val1 = val_from_counts(&count_occurences_p1(h1));
        let val2 = val_from_counts(&count_occurences_p1(h2));
        match val1.cmp(&val2) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => h1
                .chars()
                .zip(h2.chars())
                .find_map(|(c1, c2)| {
                    let n1 = map_val.get(&c1).unwrap();
                    let n2 = map_val.get(&c2).unwrap();
                    if n1 == n2 {
                        None
                    } else {
                        Some(n1.cmp(n2))
                    }
                })
                .unwrap_or(Ordering::Equal),
        }
    });
    hands_and_bids
        .into_iter()
        .enumerate()
        .map(|(i, (_, bid))| bid * (i + 1))
        .sum()
}

#[test]
fn test_process_p1() {
    let path = std::env::temp_dir().join("test_p1.dat");
    std::fs::write(
        &path,
        "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
    )
    .unwrap();
    assert_eq!(process_p1(path), 6440)
}

const CARDS_P2: [char; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

fn count_occurences_p2(hand: &str) -> HashMap<char, usize> {
    let mut counts = HashMap::from_iter(CARDS_P2.iter().map(|c| (*c, 0)));
    hand.chars().for_each(|c| {
        *counts.get_mut(&c).unwrap() += 1;
    });
    let jcount = counts.remove(&'J').unwrap();
    if jcount > 0 {
        let maxkey = *counts.iter().max_by_key(|(_, v)| *v).unwrap().0;
        *counts.get_mut(&maxkey).unwrap() += jcount
    }
    counts
}

fn process_p2(path: impl AsRef<Path>) -> usize {
    let map_val: HashMap<char, usize> =
        HashMap::from_iter(CARDS_P2.iter().enumerate().map(|(i, c)| (*c, i + 1)));
    let input = read_to_string(path).unwrap();
    let mut hands_and_bids = input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            (hand, bid.parse::<usize>().unwrap())
        })
        .collect::<Vec<_>>();
    hands_and_bids.sort_by(|(h1, _), (h2, _)| {
        let val1 = val_from_counts(&count_occurences_p2(h1));
        let val2 = val_from_counts(&count_occurences_p2(h2));
        match val1.cmp(&val2) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => h1
                .chars()
                .zip(h2.chars())
                .find_map(|(c1, c2)| {
                    let n1 = map_val.get(&c1).unwrap();
                    let n2 = map_val.get(&c2).unwrap();
                    if n1 == n2 {
                        None
                    } else {
                        Some(n1.cmp(n2))
                    }
                })
                .unwrap_or(Ordering::Equal),
        }
    });
    hands_and_bids
        .into_iter()
        .enumerate()
        .map(|(i, (_, bid))| bid * (i + 1))
        .sum()
}

#[test]
fn test_process_p2() {
    let path = std::env::temp_dir().join("test_p2.dat");
    std::fs::write(
        &path,
        "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
    )
    .unwrap();
    assert_eq!(process_p2(path), 5905)
}

fn main() {
    let t0 = Instant::now();
    let result_p1 = process_p1("data/day7.txt");
    let t1 = Instant::now();
    let result_p2 = process_p2("data/day7.txt");
    let t2 = Instant::now();
    println!("The result of p1 is {}. ({:?})", result_p1, t1 - t0);
    println!("The result of p2 is {}. ({:?})", result_p2, t2 - t1);
}
