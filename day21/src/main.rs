use ndarray::Array2;
use std::{collections::HashSet, time::Instant};

#[cfg(test)]
const TEST_CASE: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

fn process_pattern(s: &str) -> (Array2<bool>, (usize, usize)) {
    let shape = [s.lines().count(), s.lines().next().unwrap().len()];
    let mut ind = (0, 0);
    let array = Array2::from_shape_vec(
        shape,
        s.lines()
            .enumerate()
            .flat_map(|(ir, s)| s.chars().enumerate().map(move |(ic, c)| ((ir, ic), c)))
            .map(|((ir, ic), c)| match c {
                '#' => false,
                'S' => {
                    ind = (ir, ic);
                    true
                }
                _ => true,
            })
            .collect(),
    )
    .unwrap();
    (array, ind)
}

fn process_p1(data: &str, nstep: usize) -> usize {
    let (grid, (i, j)) = process_pattern(data);
    (0..nstep)
        .fold(HashSet::from([(i as i32, j as i32)]), |positions, _| {
            positions
                .into_iter()
                .flat_map(|(i, j)| {
                    [(i + 1, j), (i, j + 1), (i - 1, j), (i, j - 1)]
                        .into_iter()
                        .filter(|&(i, j)| {
                            i >= 0
                                && i < grid.shape()[0] as i32
                                && j >= 0
                                && j < grid.shape()[1] as i32
                                && grid[(i as usize, j as usize)]
                        })
                })
                .collect()
        })
        .len()
}

#[test]
fn test_process_p1() {
    assert_eq!(process_p1(TEST_CASE, 6), 16)
}

/// Part 2 is boring. It requires doing some interpolation of the data..

fn main() {
    let data = std::fs::read_to_string("data/day22.txt").unwrap();
    let t0 = Instant::now();
    let result_p1 = process_p1(&data, 64);
    let t1 = Instant::now();
    println!("The result of p1 is {}. ({:?})", result_p1, t1 - t0);
}
