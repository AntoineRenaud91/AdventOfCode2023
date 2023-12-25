use std::{collections::HashMap, time::Instant};

#[cfg(test)]
const TEST_CASE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

fn hash_string(string: &str) -> usize {
    string
        .chars()
        .fold(0, |hash, c| (hash + c as usize) * 17 % 256)
}

#[test]
fn test_hash_string() {
    assert_eq!(hash_string("HASH"), 52)
}

fn process_p1(data: &str) -> usize {
    data.split(',').map(hash_string).sum()
}

#[test]
fn test_process_p1() {
    assert_eq!(process_p1(TEST_CASE), 1320)
}

#[derive(Debug)]
enum Operation {
    Remove((usize, String)),
    Add((usize, String, usize)),
}

fn process_p2(data: &str) -> usize {
    data.split(',')
        .map(|s| match s.split_once('=') {
            Some((label, focal)) => {
                Operation::Add((hash_string(label), label.to_owned(), focal.parse().unwrap()))
            }
            None => {
                Operation::Remove((hash_string(&s[..s.len() - 1]), s[..s.len() - 1].to_owned()))
            }
        })
        .fold(
            HashMap::<usize, Vec<(String, usize)>>::new(),
            |mut map, op| {
                match op {
                    Operation::Add((hash, label, focal)) => {
                        if let Some(array) = map.get_mut(&hash) {
                            if let Some(i) =
                                array.iter().enumerate().find_map(|(i, (clabel, _))| {
                                    if &label == clabel {
                                        Some(i)
                                    } else {
                                        None
                                    }
                                })
                            {
                                array[i].1 = focal
                            } else {
                                array.push((label, focal));
                            }
                        } else {
                            map.insert(hash, vec![(label, focal)]);
                        };
                    }
                    Operation::Remove((hash, label)) => {
                        if let Some(array) = map.get_mut(&hash) {
                            if let Some(i) =
                                array.iter().enumerate().find_map(|(i, (clabel, _))| {
                                    if &label == clabel {
                                        Some(i)
                                    } else {
                                        None
                                    }
                                })
                            {
                                array.remove(i);
                            }
                        }
                    }
                }
                map
            },
        )
        .into_iter()
        .flat_map(|(i, array)| {
            array
                .into_iter()
                .enumerate()
                .map(move |(j, (_, focal))| (i + 1) * (j + 1) * focal)
        })
        .sum()
}

#[test]
fn test_process_p2() {
    assert_eq!(process_p2(TEST_CASE), 145)
}

fn main() {
    let data = std::fs::read_to_string("data/day15.txt").unwrap();
    let t0 = Instant::now();
    let result_p1 = process_p1(&data);
    let t1 = Instant::now();
    println!("The result of p1 is {}. ({:?})", result_p1, t1 - t0);
    let result_p2 = process_p2(&data);
    let t2 = Instant::now();
    println!("The result of p2 is {}. ({:?})", result_p2, t2 - t1);
}
