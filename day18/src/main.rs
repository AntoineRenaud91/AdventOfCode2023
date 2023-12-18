use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    time::Instant,
};

use geo::{Area, LineString, Polygon};

#[derive(Debug, PartialEq, Eq)]
enum Dir {
    L,
    R,
    U,
    D,
}

impl From<char> for Dir {
    fn from(value: char) -> Self {
        match value {
            'D' | '1' => Self::D,
            'R' | '0' => Self::R,
            'L' | '2' => Self::L,
            'U' | '3' => Self::U,
            _ => panic!("not possible"),
        }
    }
}

fn count_inner(line: Vec<(i32, i32)>) -> usize {
    let boundary_count = line
        .windows(2)
        .map(|slice| {
            let (i1, j1) = slice[0];
            let (i2, j2) = slice[1];
            ((i1 - i2).abs() + (j1 - j2).abs()) as usize
        })
        .sum::<usize>();
    let area = Polygon::new(
        LineString::from_iter(line.into_iter().map(|(i, j)| (i as f64, j as f64))),
        vec![],
    )
    .unsigned_area()
    .round() as usize;
    area + boundary_count / 2 + 1
}

fn process_p1(path: impl AsRef<Path>) -> usize {
    let line = BufReader::new(File::open(path).unwrap())
        .lines()
        .flatten()
        .map(|line| {
            let mut iter = line.split_whitespace();
            let dir: Dir = iter.next().unwrap().chars().next().unwrap().into();
            let n = iter.next().unwrap().parse::<i32>().unwrap();
            (dir, n)
        })
        .fold(vec![(0, 0)], |mut line, (dir, n)| {
            let (i, j) = *line.last().unwrap();
            match dir {
                Dir::L => line.push((i - n, j)),
                Dir::R => line.push((i + n, j)),
                Dir::U => line.push((i, j + n)),
                Dir::D => line.push((i, j - n)),
            }
            line
        });
    count_inner(line)
}

#[test]
fn test_process_p1() {
    let path = std::env::temp_dir().join("test_p1.dat");
    std::fs::write(
        &path,
        "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)",
    )
    .unwrap();
    assert_eq!(process_p1(path), 62)
}

fn parse_hex(s: &str) -> (Dir, i32) {
    (
        s.chars().nth(7).unwrap().into(),
        i32::from_str_radix(&s[2..7], 16).unwrap(),
    )
}

#[test]
fn test_parse_hex() {
    assert_eq!(parse_hex("(#70c710)"), (Dir::R, 461937));
}

fn process_p2(path: impl AsRef<Path>) -> usize {
    let line = BufReader::new(File::open(path).unwrap())
        .lines()
        .flatten()
        .map(|line| parse_hex(line.split_whitespace().nth(2).unwrap()))
        .fold(vec![(0, 0)], |mut line, (dir, n)| {
            let (i, j) = *line.last().unwrap();
            match dir {
                Dir::L => line.push((i - n, j)),
                Dir::R => line.push((i + n, j)),
                Dir::U => line.push((i, j + n)),
                Dir::D => line.push((i, j - n)),
            }
            line
        });
    count_inner(line)
}

#[test]
fn test_process_p2() {
    let path = std::env::temp_dir().join("test_p2.dat");
    std::fs::write(
        &path,
        "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)",
    )
    .unwrap();
    assert_eq!(process_p2(path), 952408144115)
}

fn main() {
    let t0 = Instant::now();
    let result_p1 = process_p1("data/day18.txt");
    let t1 = Instant::now();
    println!("The result of p1 is {}. ({:?})", result_p1, t1 - t0);
    let result_p2 = process_p2("data/day18.txt");
    let t2 = Instant::now();
    println!("The result of p2 is {}. ({:?})", result_p2, t2 - t1);
}
