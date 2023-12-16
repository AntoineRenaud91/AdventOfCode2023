use std::{collections::HashSet, path::Path, time::Instant};

use ndarray::{concatenate, Array2, Axis};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn process_pattern(pattern: &str) -> Array2<char> {
    let ncols = pattern.lines().next().unwrap().len();
    pattern.lines().fold(
        Array2::from_shape_vec([0, ncols], vec![]).unwrap(),
        |pattern, line| {
            concatenate![
                Axis(0),
                pattern.view(),
                Array2::from_shape_vec([1, ncols], line.chars().collect::<Vec<_>>())
                    .unwrap()
                    .view()
            ]
        },
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    T,
    R,
    B,
    L,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Beam {
    pos: (usize, usize),
    dir: Dir,
}

impl Beam {
    fn energize_grid(&self, egrid: &mut Array2<bool>) {
        egrid[self.pos] = true
    }

    fn apply(self, grid: &Array2<char>) -> Vec<Self> {
        let Self { pos, dir } = self;
        match (grid[pos], dir) {
            ('/', Dir::R) => vec![Self { pos, dir: Dir::T }],
            ('/', Dir::B) => vec![Self { pos, dir: Dir::L }],
            ('/', Dir::L) => vec![Self { pos, dir: Dir::B }],
            ('/', Dir::T) => vec![Self { pos, dir: Dir::R }],
            ('\\', Dir::R) => vec![Self { pos, dir: Dir::B }],
            ('\\', Dir::B) => vec![Self { pos, dir: Dir::R }],
            ('\\', Dir::L) => vec![Self { pos, dir: Dir::T }],
            ('\\', Dir::T) => vec![Self { pos, dir: Dir::L }],
            ('|', Dir::L) | ('|', Dir::R) => {
                vec![Self { pos, dir: Dir::T }, Self { pos, dir: Dir::B }]
            }
            ('-', Dir::T) | ('-', Dir::B) => {
                vec![Self { pos, dir: Dir::R }, Self { pos, dir: Dir::L }]
            }
            _ => vec![Self { pos, dir }],
        }
    }
    fn slide(self, grid_shape: &[usize]) -> Option<Self> {
        let Self {
            pos: (mut i, mut j),
            dir,
        } = self;
        match dir {
            Dir::L => j = j.checked_sub(1)?,
            Dir::R => {
                j += 1;
                if j == grid_shape[1] {
                    return None;
                }
            }
            Dir::T => i = i.checked_sub(1)?,
            Dir::B => {
                i += 1;
                if i == grid_shape[0] {
                    return None;
                }
            }
        };
        Some(Self { pos: (i, j), dir })
    }
}

fn get_energized_grid(
    grid: &Array2<char>,
    mut egrid: Array2<bool>,
    mut beams: Vec<Beam>,
    mut memo: HashSet<Beam>,
) -> Array2<bool> {
    if beams.is_empty() {
        return egrid;
    };
    beams = beams
        .into_iter()
        .flat_map(|b| {
            b.energize_grid(&mut egrid);
            b.apply(grid)
        })
        .filter_map(|mut b| {
            b = b.slide(grid.shape())?;
            if memo.insert(b) {
                Some(b)
            } else {
                None
            }
        })
        .collect();
    get_energized_grid(grid, egrid, beams, memo)
}

fn process_p1(path: impl AsRef<Path>) -> usize {
    let grid = process_pattern(&std::fs::read_to_string(path).unwrap());
    let egrid = get_energized_grid(
        &grid,
        Array2::<bool>::from_elem([grid.shape()[0], grid.shape()[1]], false),
        vec![Beam {
            pos: (0, 0),
            dir: Dir::R,
        }],
        HashSet::new(),
    );
    egrid.into_iter().filter(|&c| c).count()
}

#[test]
fn test_process_p1() {
    let path = std::env::temp_dir().join("test_p1.dat");
    std::fs::write(
        &path,
        r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....",
    )
    .unwrap();
    assert_eq!(process_p1(path), 51)
}

fn process_p2(path: impl AsRef<Path>) -> usize {
    let grid = process_pattern(&std::fs::read_to_string(path).unwrap());
    let nrows = grid.shape()[0];
    let ncols = grid.shape()[1];
    let init_beams = (0..nrows)
        .flat_map(|i| {
            vec![
                Beam {
                    pos: (i, 0),
                    dir: Dir::R,
                },
                Beam {
                    pos: (i, ncols - 1),
                    dir: Dir::L,
                },
            ]
        })
        .chain((0..nrows).flat_map(|j| {
            vec![
                Beam {
                    pos: (0, j),
                    dir: Dir::B,
                },
                Beam {
                    pos: (nrows - 1, j),
                    dir: Dir::T,
                },
            ]
        }))
        .collect::<Vec<_>>();
    init_beams
        .into_par_iter()
        .map(|b| {
            get_energized_grid(
                &grid,
                Array2::<bool>::from_elem([grid.shape()[0], grid.shape()[1]], false),
                vec![b],
                HashSet::new(),
            )
            .into_iter()
            .filter(|&b| b)
            .count()
        })
        .max()
        .unwrap()
}

#[test]
fn test_process_p2() {
    let path = std::env::temp_dir().join("test_p2.dat");
    std::fs::write(
        &path,
        r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....",
    )
    .unwrap();
    assert_eq!(process_p2(path), 51)
}

fn main() {
    let t0 = Instant::now();
    let result_p1 = process_p1("data/day16.txt");
    let t1 = Instant::now();
    println!("The result of p1 is {}. ({:?})", result_p1, t1 - t0);
    let result_p2 = process_p2("data/day16.txt");
    let t2 = Instant::now();
    println!("The result of p2 is {}. ({:?})", result_p2, t2 - t1);
}
