use ndarray::{concatenate, Array1, Array2, Axis};
use std::{collections::HashMap, time::Instant};

#[cfg(test)]
const TEST_CASE: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

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
                        .map(|c| match c {
                            '#' => 2,
                            'O' => 1,
                            _ => 0,
                        })
                        .collect::<Vec<_>>()
                )
                .unwrap()
                .view()
            ]
        },
    )
}

fn process_col(col: ndarray::ArrayView1<u8>) -> usize {
    col.iter()
        .enumerate()
        .fold((0, 0, 0), |(sum, cur_sum, block_index), (i, v)| match v {
            0 => (sum, cur_sum, block_index),
            1 => (sum, cur_sum + 1, block_index),
            _ => (
                sum + (0..cur_sum)
                    .map(|j| col.len() - 1 - block_index - j)
                    .sum::<usize>(),
                0,
                i + 1,
            ),
        })
        .0
}

#[test]
fn test_process_col() {
    assert_eq!(
        process_col(ndarray::array![1, 1, 0, 1, 0, 1, 0, 0, 2, 2, 2].view()),
        10 + 9 + 8 + 7
    );
    assert_eq!(
        process_col(ndarray::array![1, 1, 0, 1, 2, 1, 0, 0, 2, 2, 2].view()),
        10 + 9 + 8 + 5
    )
}

fn process_p1(data: &str) -> usize {
    let mut table = process_pattern(data);
    table
        .push_row(Array1::from_elem(table.ncols(), 2).view())
        .unwrap();
    table.columns().into_iter().map(process_col).sum()
}

#[test]
fn test_process_p1() {
    assert_eq!(process_p1(TEST_CASE), 136)
}

fn tilt_left(col: Vec<u8>) -> Vec<u8> {
    let (mut cum_array, cur_sum, _) = col.iter().enumerate().fold(
        (vec![], 0, 0),
        |(mut cum_array, cur_sum, block_index), (i, v)| match v {
            0 => (cum_array, cur_sum, block_index),
            1 => (cum_array, cur_sum + 1, block_index),
            _ => {
                cum_array.extend((0..cur_sum).map(|_| 1));
                cum_array.extend((cum_array.len()..i).map(|_| 0));
                cum_array.push(2);
                (cum_array, 0, i + 1)
            }
        },
    );
    cum_array.extend((0..cur_sum).map(|_| 1));
    cum_array.extend((cum_array.len()..col.len()).map(|_| 0));
    cum_array
}

#[test]
fn test_tilt_col() {
    assert_eq!(
        tilt_left(vec![1, 1, 0, 1, 0, 1, 0, 0, 2, 2]),
        vec![1, 1, 1, 1, 0, 0, 0, 0, 2, 2],
    );
    assert_eq!(
        tilt_left(vec![2, 1, 0, 1, 2, 1, 2, 0, 2, 1]),
        vec![2, 1, 1, 0, 2, 1, 2, 0, 2, 1]
    )
}

fn tilt_all_north(mut table: Array2<u8>) -> Array2<u8> {
    table.columns_mut().into_iter().for_each(|mut col| {
        let col_vec = col.to_vec();
        col.iter_mut()
            .zip(tilt_left(col_vec))
            .for_each(|(a, b)| *a = b)
    });
    table
}

#[test]
fn test_tilt_north() {
    assert_eq!(
        tilt_all_north(ndarray::array![
            [0, 1, 2, 0, 1],
            [1, 0, 0, 0, 0],
            [2, 0, 1, 0, 1],
            [1, 0, 0, 1, 0]
        ]),
        ndarray::array![
            [1, 1, 2, 1, 1],
            [0, 0, 1, 0, 1],
            [2, 0, 0, 0, 0],
            [1, 0, 0, 0, 0]
        ]
    );
}

fn tilt_all_south(mut table: Array2<u8>) -> Array2<u8> {
    table.columns_mut().into_iter().for_each(|mut col| {
        let mut col_vec = col.to_vec();
        col_vec.reverse();
        let mut new_col = tilt_left(col_vec);
        new_col.reverse();
        col.iter_mut().zip(new_col).for_each(|(a, b)| *a = b)
    });
    table
}

#[test]
fn test_tilt_south() {
    assert_eq!(
        tilt_all_south(ndarray::array![
            [0, 1, 2, 0, 1],
            [1, 0, 0, 0, 0],
            [2, 0, 1, 0, 1],
            [1, 0, 0, 1, 0]
        ]),
        ndarray::array![
            [0, 0, 2, 0, 0],
            [1, 0, 0, 0, 0],
            [2, 0, 0, 0, 1],
            [1, 1, 1, 1, 1]
        ]
    );
}

fn tilt_all_west(mut table: Array2<u8>) -> Array2<u8> {
    table.rows_mut().into_iter().for_each(|mut row| {
        let row_vec = row.to_vec();
        row.iter_mut()
            .zip(tilt_left(row_vec))
            .for_each(|(a, b)| *a = b)
    });
    table
}

#[test]
fn test_tilt_west() {
    assert_eq!(
        tilt_all_west(ndarray::array![
            [0, 1, 2, 0, 1],
            [1, 0, 0, 0, 0],
            [2, 0, 1, 0, 1],
            [1, 0, 0, 1, 0]
        ]),
        ndarray::array![
            [1, 0, 2, 1, 0],
            [1, 0, 0, 0, 0],
            [2, 1, 1, 0, 0],
            [1, 1, 0, 0, 0]
        ]
    )
}

fn tilt_all_east(mut table: Array2<u8>) -> Array2<u8> {
    table.rows_mut().into_iter().for_each(|mut row| {
        let mut row_vec = row.to_vec();
        row_vec.reverse();
        let mut new_row = tilt_left(row_vec);
        new_row.reverse();
        row.iter_mut().zip(new_row).for_each(|(a, b)| *a = b)
    });
    table
}

#[test]
fn test_tilt_east() {
    assert_eq!(
        tilt_all_east(ndarray::array![
            [0, 1, 2, 0, 1],
            [1, 0, 0, 0, 0],
            [2, 0, 1, 0, 1],
            [1, 0, 0, 1, 0]
        ]),
        ndarray::array![
            [0, 1, 2, 0, 1],
            [0, 0, 0, 0, 1],
            [2, 0, 0, 1, 1],
            [0, 0, 0, 1, 1]
        ]
    )
}

fn tilt_cycle(table: Array2<u8>) -> Array2<u8> {
    tilt_all_east(tilt_all_south(tilt_all_west(tilt_all_north(table))))
}

#[test]
fn test_tilt_cycle() {
    assert_eq!(
        tilt_cycle(ndarray::array![
            [1, 0, 0, 0, 0, 2, 0, 0, 0, 0],
            [1, 0, 1, 1, 2, 0, 0, 0, 0, 2],
            [0, 0, 0, 0, 0, 2, 2, 0, 0, 0],
            [1, 1, 0, 2, 1, 0, 0, 0, 0, 1],
            [0, 1, 0, 0, 0, 0, 0, 1, 2, 0],
            [1, 0, 2, 0, 0, 1, 0, 2, 0, 2],
            [0, 0, 1, 0, 0, 2, 1, 0, 0, 1],
            [0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            [2, 0, 0, 0, 0, 2, 2, 2, 0, 0],
            [2, 1, 1, 0, 0, 2, 0, 0, 0, 0]
        ]),
        ndarray::array![
            [0, 0, 0, 0, 0, 2, 0, 0, 0, 0],
            [0, 0, 0, 0, 2, 0, 0, 0, 1, 2],
            [0, 0, 0, 1, 1, 2, 2, 0, 0, 0],
            [0, 1, 1, 2, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 1, 1, 1, 2, 0],
            [0, 1, 2, 0, 0, 0, 1, 2, 0, 2],
            [0, 0, 0, 0, 1, 2, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 1, 1, 1, 1],
            [2, 0, 0, 0, 1, 2, 2, 2, 0, 0],
            [2, 0, 0, 1, 1, 2, 0, 0, 0, 0]
        ]
    );
}

fn table_load(table: &Array2<u8>) -> usize {
    table
        .rows()
        .into_iter()
        .enumerate()
        .map(|(irow, row)| row.iter().filter(|v| v == &&1).count() * (table.ncols() - irow))
        .sum()
}

fn process_p2(data: &str, n: usize) -> usize {
    let mut table = process_pattern(data);
    let mut results = vec![table_load(&table)];
    let mut tmap = HashMap::new();
    let mut i = 0;
    while i != n {
        if let Some(j) = tmap.get(&table) {
            return results[j + (n - i) % (i - j) + 1];
        }
        results.push(table_load(&table));
        tmap.insert(table.clone(), i);
        table = tilt_cycle(table.clone());
        i += 1
    }
    panic!("error");
}

#[test]
fn test_process_p2() {
    assert_eq!(process_p2(TEST_CASE, 1000), 64)
}

fn main() {
    let data = std::fs::read_to_string("data/day14.txt").unwrap();
    let t0 = Instant::now();
    let result_p1 = process_p1(&data);
    let t1 = Instant::now();
    println!("The result of p1 is {}. ({:?})", result_p1, t1 - t0);
    let result_p2 = process_p2(&data, 1000);
    let t2 = Instant::now();
    println!("The result of p2 is {}. ({:?})", result_p2, t2 - t1);
}
