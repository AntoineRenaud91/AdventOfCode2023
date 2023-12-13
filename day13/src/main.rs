use std::{path::Path, time::Instant};

use ndarray::{array, concatenate, s, Array2, Axis};

fn process_pattern(pattern: &str) -> Array2<u8> {
    let ncols = pattern.lines().next().unwrap().len();
    pattern.lines().fold(
        Array2::<u8>::from_shape_vec([0, ncols], vec![]).unwrap(),
        |pattern, line| {
            concatenate![
                Axis(0),
                pattern.view(),
                Array2::from_shape_vec(
                    [1, ncols],
                    line.chars()
                        .map(|c| if c == '#' { 1 } else { 0 })
                        .collect::<Vec<_>>()
                )
                .unwrap()
                .view()
            ]
        },
    )
}

fn find_horizontal_reflexion(pattern: &Array2<u8>) -> Option<usize> {
    let nrows = pattern.shape()[0];
    (1..nrows)
        .find(|irow| {
            (1..(irow + 1))
                .take_while(|i| irow + i - 1 < nrows)
                .all(|i| pattern.slice(s![irow - i, ..]) == pattern.slice(s![irow + i - 1, ..]))
        })
        .map(|v| v * 100)
}

#[test]
fn test_find_horizontal() {
    let pattern = array![
        [0, 1, 0, 1, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 1, 0, 1, 0],
    ];
    assert_eq!(find_horizontal_reflexion(&pattern), Some(200))
}

fn find_vertical_reflexion(pattern: &Array2<u8>) -> Option<usize> {
    let ncols = pattern.shape()[1];
    (1..ncols).find(|icol| {
        (1..(icol + 1))
            .take_while(|i| icol + i - 1 < ncols)
            .all(|i| pattern.slice(s![.., icol - i]) == pattern.slice(s![.., icol + i - 1]))
    })
}

#[test]
fn test_find_vertical() {
    let pattern = array![
        [0, 1, 0, 1, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 1, 0, 1, 0],
    ];
    assert!(find_vertical_reflexion(&pattern).is_none());
    let pattern = array![
        [0, 1, 0, 0, 1],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 1, 0, 0, 1],
    ];
    assert_eq!(find_vertical_reflexion(&pattern), Some(3))
}

fn process_p1(path: impl AsRef<Path>) -> usize {
    std::fs::read_to_string(path)
        .unwrap()
        .split("\n\n")
        .map(process_pattern)
        .map(|pattern| {
            find_horizontal_reflexion(&pattern)
                .unwrap_or_else(|| find_vertical_reflexion(&pattern).unwrap())
        })
        .sum()
}

#[test]
fn test_process_p1() {
    let path = std::env::temp_dir().join("test_p1.dat");
    std::fs::write(
        &path,
        "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
    )
    .unwrap();
    assert_eq!(process_p1(path), 405)
}

fn find_horizontal_reflexion_with_smudge(pattern: &Array2<u8>) -> Option<usize> {
    let nrows = pattern.shape()[0];
    (1..nrows)
        .find(|irow| {
            let mut corrected = false;
            (1..(irow + 1))
                .take_while(|i| irow + i - 1 < nrows)
                .all(|i| {
                    let p1 = pattern.slice(s![irow - i, ..]);
                    let p2 = pattern.slice(s![irow + i - 1, ..]);
                    if p1 == p2 {
                        return true;
                    }
                    if corrected {
                        return false;
                    }
                    if p1.iter().zip(p2).map(|(v1, v2)| (v1 + v2) % 2).sum::<u8>() == 1 {
                        corrected = true;
                        return true;
                    }
                    false
                })
                && corrected
        })
        .map(|v| v * 100)
}

#[test]
fn test_find_horizontal_smudge() {
    let pattern = array![
        [0, 1, 0, 1, 0],
        [0, 0, 0, 0, 1],
        [0, 0, 0, 0, 1],
        [0, 1, 0, 0, 0],
    ];
    assert_eq!(find_horizontal_reflexion_with_smudge(&pattern), Some(200))
}

fn find_vertical_reflexion_with_smudge(pattern: &Array2<u8>) -> Option<usize> {
    let ncols = pattern.shape()[1];
    (1..ncols).find(|icol| {
        let mut corrected = false;
        (1..(icol + 1))
            .take_while(|i| icol + i - 1 < ncols)
            .all(|i| {
                let p1 = pattern.slice(s![.., icol - i]);
                let p2 = pattern.slice(s![.., icol + i - 1]);
                if p1 == p2 {
                    return true;
                }
                if corrected {
                    return false;
                }
                if p1.iter().zip(p2).map(|(v1, v2)| (v1 + v2) % 2).sum::<u8>() == 1 {
                    corrected = true;
                    return true;
                }
                false
            })
            && corrected
    })
}

fn process_p2(path: impl AsRef<Path>) -> usize {
    std::fs::read_to_string(path)
        .unwrap()
        .split("\n\n")
        .map(process_pattern)
        .map(|pattern| {
            find_horizontal_reflexion_with_smudge(&pattern)
                .unwrap_or_else(|| find_vertical_reflexion_with_smudge(&pattern).unwrap())
        })
        .sum()
}

#[test]
fn test_process_p2() {
    let path = std::env::temp_dir().join("test_p2.dat");
    std::fs::write(
        &path,
        "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
    )
    .unwrap();
    assert_eq!(process_p2(path), 400)
}

fn main() {
    let t0 = Instant::now();
    let result_p1 = process_p1("data/day13.txt");
    let t1 = Instant::now();
    println!("The result of p1 is {}. ({:?})", result_p1, t1 - t0);
    let result_p2 = process_p2("data/day13.txt");
    let t2 = Instant::now();
    println!("The result of p2 is {}. ({:?})", result_p2, t2 - t1);
}
