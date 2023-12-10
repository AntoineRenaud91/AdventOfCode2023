use std::{path::Path, time::Instant};

use geo::{Contains, Coord, LineString, Polygon};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum From {
    Top,
    Right,
    Bottom,
    Left,
}

fn length_to_s(
    (i, j): (usize, usize),
    count: usize,
    from: From,
    grid: &[Vec<char>],
) -> Option<usize> {
    match (grid[i][j], from) {
        ('S', _) => Some(count),
        ('.', _) => None,
        ('|', From::Top) | ('7', From::Left) | ('F', From::Right) => {
            if i == grid.len() - 1 {
                None
            } else {
                length_to_s((i + 1, j), count + 1, From::Top, grid)
            }
        }
        ('|', From::Bottom) | ('J', From::Left) | ('L', From::Right) => {
            if i == 0 {
                None
            } else {
                length_to_s((i - 1, j), count + 1, From::Bottom, grid)
            }
        }
        ('L', From::Top) | ('-', From::Left) | ('F', From::Bottom) => {
            if j == grid[0].len() - 1 {
                None
            } else {
                length_to_s((i, j + 1), count + 1, From::Left, grid)
            }
        }
        ('J', From::Top) | ('-', From::Right) | ('7', From::Bottom) => {
            if j == 0 {
                None
            } else {
                length_to_s((i, j - 1), count + 1, From::Right, grid)
            }
        }
        _ => None,
    }
}

fn process_p1(path: impl AsRef<Path>) -> usize {
    let grid = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|lines| lines.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let (i, j) = grid
        .iter()
        .enumerate()
        .find_map(|(i, rows)| {
            rows.iter()
                .enumerate()
                .find_map(|(j, c)| if c == &'S' { Some((i, j)) } else { None })
        })
        .unwrap();
    *[
        {
            if i == 0 {
                None
            } else {
                length_to_s((i - 1, j), 1, From::Bottom, &grid)
            }
        },
        {
            if i == grid.len() - 1 {
                None
            } else {
                length_to_s((i + 1, j), 1, From::Top, &grid)
            }
        },
        {
            if j == 0 {
                None
            } else {
                length_to_s((i, j - 1), 1, From::Right, &grid)
            }
        },
        {
            if j == grid[0].len() - 1 {
                None
            } else {
                length_to_s((i, j + 1), 1, From::Left, &grid)
            }
        },
    ]
    .iter()
    .flatten()
    .max()
    .unwrap()
        / 2
}

#[test]
fn test_process_p1() {
    let path = std::env::temp_dir().join("test_p1.dat");
    std::fs::write(
        &path,
        "..F7.
.FJ|.
SJ.L7
|F--J
LJ...",
    )
    .unwrap();
    assert_eq!(process_p1(path), 8)
}

fn get_loop(
    (i, j): (usize, usize),
    mut curr_loop: Vec<(i32, i32)>,
    from: From,
    grid: &[Vec<char>],
) -> Option<Vec<(i32, i32)>> {
    curr_loop.push((i as i32, j as i32));
    match (grid[i][j], from) {
        ('S', _) => Some(curr_loop),
        ('.', _) => None,
        ('|', From::Top) | ('7', From::Left) | ('F', From::Right) => {
            if i == grid.len() - 1 {
                None
            } else {
                get_loop((i + 1, j), curr_loop, From::Top, grid)
            }
        }
        ('|', From::Bottom) | ('J', From::Left) | ('L', From::Right) => {
            if i == 0 {
                None
            } else {
                get_loop((i - 1, j), curr_loop, From::Bottom, grid)
            }
        }
        ('L', From::Top) | ('-', From::Left) | ('F', From::Bottom) => {
            if j == grid[0].len() - 1 {
                None
            } else {
                get_loop((i, j + 1), curr_loop, From::Left, grid)
            }
        }
        ('J', From::Top) | ('-', From::Right) | ('7', From::Bottom) => {
            if j == 0 {
                None
            } else {
                get_loop((i, j - 1), curr_loop, From::Right, grid)
            }
        }
        _ => None,
    }
}

fn process_p2(path: impl AsRef<Path>) -> usize {
    let grid = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|lines| lines.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let (i, j) = grid
        .iter()
        .enumerate()
        .find_map(|(i, rows)| {
            rows.iter()
                .enumerate()
                .find_map(|(j, c)| if c == &'S' { Some((i, j)) } else { None })
        })
        .unwrap();
    let main_loop = [
        {
            if i == 0 {
                None
            } else {
                get_loop((i - 1, j), vec![], From::Bottom, &grid)
            }
        },
        {
            if i == grid.len() - 1 {
                None
            } else {
                get_loop((i + 1, j), vec![], From::Top, &grid)
            }
        },
        {
            if j == 0 {
                None
            } else {
                get_loop((i, j - 1), vec![], From::Right, &grid)
            }
        },
        {
            if j == grid[0].len() - 1 {
                None
            } else {
                get_loop((i, j + 1), vec![], From::Left, &grid)
            }
        },
    ]
    .into_iter()
    .flatten()
    .max_by_key(|c| c.len())
    .unwrap();
    let polygon = Polygon::new(LineString::from(main_loop), vec![]);
    (1..(grid.len() - 1))
        .flat_map(|i| (1..(grid[0].len() - 1)).map(move |j| (i as i32, j as i32)))
        .filter(|(i, j)| polygon.contains(&Coord { x: *i, y: *j }))
        .count()
}

#[test]
fn test_process_p2() {
    let path = std::env::temp_dir().join("test_p2.dat");
    std::fs::write(
        &path,
        "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJIF7FJ-
L---JF-JLJIIIIFJLJJ7
|F|F-JF---7IIIL7L|7|
|FFJF7L7F-JF7IIL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L",
    )
    .unwrap();
    assert_eq!(process_p2(path), 10)
}

fn main() {
    let t0 = Instant::now();
    let result_p1 = process_p1("data/day10.txt");
    let t1 = Instant::now();
    let result_p2 = process_p2("data/day10.txt");
    let t2 = Instant::now();
    println!("The result of p1 is {}. ({:?})", result_p1, t1 - t0);
    println!("The result of p2 is {}. ({:?})", result_p2, t2 - t1);
}
