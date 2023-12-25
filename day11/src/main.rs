use std::time::Instant;

use ndarray::{Array1, Array2, Axis};

#[cfg(test)]
const TEST_CASE: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

fn process_p1(data: &str) -> i32 {
    let ncol = data.lines().next().unwrap().len();
    let mut universe = Array2::from_shape_vec([0, ncol], vec![]).unwrap();
    for lines in data.lines() {
        let row_vec = lines
            .chars()
            .map(|c| if c == '#' { 1 } else { 0 })
            .collect::<Vec<_>>();
        if !row_vec.iter().any(|c| c == &1) {
            universe
                .push_row(Array1::from(row_vec.clone()).view())
                .unwrap();
        }
        universe.push_row(Array1::from(row_vec).view()).unwrap();
    }
    let universe = {
        let nrows = universe.shape()[0];
        let mut expanded_universe = Array2::from_shape_vec([nrows, 0], vec![]).unwrap();
        for col in universe.axis_iter(Axis(1)) {
            if !col.iter().any(|v| v == &1) {
                expanded_universe.push_column(col).unwrap()
            }
            expanded_universe.push_column(col).unwrap()
        }
        expanded_universe
    };
    let [nrows, ncols] = [universe.shape()[0], universe.shape()[1]] as [usize; 2];
    let galaxies = (0..nrows)
        .flat_map(|i| (0..ncols).map(move |j| (i, j)))
        .filter(|ind| universe[*ind] == 1)
        .map(|(i, j)| (i as i32, j as i32))
        .collect::<Vec<_>>();
    (0..(galaxies.len() - 1))
        .flat_map(|n| ((n + 1)..galaxies.len()).map(move |m| (n, m)))
        .map(|(n, m)| {
            let (i_n, j_n) = galaxies[n];
            let (i_m, j_m) = galaxies[m];
            (i_n - i_m).abs() + (j_n - j_m).abs()
        })
        .sum()
}

#[test]
fn test_process_p1() {
    assert_eq!(process_p1(TEST_CASE), 374)
}

fn process_p2(data: &str, exp_factor: i64) -> i64 {
    let ncol = data.lines().next().unwrap().len();
    let mut universe = Array2::from_shape_vec([0, ncol], vec![]).unwrap();
    let mut empty_rows = vec![];
    for (i, lines) in data.lines().enumerate() {
        let row_vec = lines
            .chars()
            .map(|c| if c == '#' { 1 } else { 0 })
            .collect::<Vec<_>>();
        if !row_vec.iter().any(|c| c == &1) {
            empty_rows.push(i)
        }
        universe.push_row(Array1::from(row_vec).view()).unwrap();
    }
    let empty_cols = universe
        .axis_iter(Axis(1))
        .enumerate()
        .filter_map(|(j, col)| {
            if col.iter().any(|v| v == &1) {
                None
            } else {
                Some(j)
            }
        })
        .collect::<Vec<_>>();
    let [nrows, ncols] = [universe.shape()[0], universe.shape()[1]] as [usize; 2];
    let galaxies = (0..nrows)
        .flat_map(|i| (0..ncols).map(move |j| (i, j)))
        .filter(|ind| universe[*ind] == 1)
        .map(|(i, j)| (i as i64, j as i64))
        .collect::<Vec<_>>();
    (0..(galaxies.len() - 1))
        .flat_map(|n| ((n + 1)..galaxies.len()).map(move |m| (n, m)))
        .map(|(n, m)| {
            let (i_n, j_n) = galaxies[n];
            let (i_m, j_m) = galaxies[m];
            let r_min = empty_rows.binary_search(&(i_n as usize)).unwrap_err() as i64;
            let r_max = empty_rows.binary_search(&(i_m as usize)).unwrap_err() as i64;
            let c_min = empty_cols.binary_search(&(j_n as usize)).unwrap_err() as i64;
            let c_max = empty_cols.binary_search(&(j_m as usize)).unwrap_err() as i64;
            (i_n - i_m).abs()
                + (j_n - j_m).abs()
                + ((r_max - r_min).abs() + (c_max - c_min).abs()) * (exp_factor - 1)
        })
        .sum()
}

#[test]
fn test_process_p2() {
    assert_eq!(process_p2(TEST_CASE, 10), 1030)
}

fn main() {
    let data = std::fs::read_to_string("data/day11.txt").unwrap();
    let t0 = Instant::now();
    let result_p1 = process_p1(&data);
    let t1 = Instant::now();
    println!("The result of p1 is {}. ({:?})", result_p1, t1 - t0);
    let result_p2 = process_p2(&data, 1000000);
    let t2 = Instant::now();
    println!("The result of p2 is {}. ({:?})", result_p2, t2 - t1);
}
