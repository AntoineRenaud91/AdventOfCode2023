use std::{cmp::Reverse, collections::HashSet, time::Instant};

use ndarray::{concatenate, Array2, Axis};
use std::collections::BinaryHeap;
use std::hash::Hash;

#[cfg(test)]
const TEST_CASE: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

fn process_pattern(pattern: &str) -> Array2<usize> {
    let ncols = pattern.lines().next().unwrap().len();
    pattern.lines().filter(|line| !line.is_empty()).fold(
        Array2::from_shape_vec([0, ncols], vec![]).unwrap(),
        |pattern, line| {
            concatenate![
                Axis(0),
                pattern.view(),
                Array2::from_shape_vec(
                    [1, ncols],
                    line.chars()
                        .map(|c| c.to_digit(10).unwrap() as usize)
                        .collect::<Vec<_>>()
                )
                .unwrap()
                .view()
            ]
        },
    )
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Dir {
    T(usize),
    L(usize),
    B(usize),
    R(usize),
}

impl Dir {
    fn unwrap_value(self) -> usize {
        match self {
            Self::T(c) | Self::L(c) | Self::B(c) | Self::R(c) => c,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Node {
    pos: (usize, usize, Dir),
    prev: Option<(usize, usize, Dir)>,
    cost: usize,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cost.cmp(&other.cost))
    }
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

fn process_p1(data: &str) -> usize {
    let city = process_pattern(data);
    let nx = city.shape()[0];
    let ny = city.shape()[1];
    let source = (0, 0);
    let target = (nx - 1, ny - 1);
    let mut explored = HashSet::new();
    let mut heap = BinaryHeap::from([
        Reverse(Node {
            pos: (source.0, source.1, Dir::L(0)),
            cost: 0,
            prev: None,
        }),
        Reverse(Node {
            pos: (source.0, source.1, Dir::B(0)),
            cost: 0,
            prev: None,
        }),
    ]);
    while let Some(Reverse(node)) = heap.pop() {
        if explored.contains(&node.pos) {
            continue;
        }
        explored.insert(node.pos);
        let (i, j, dir) = node.pos;
        if target == (i, j) {
            return node.cost;
        }
        match dir {
            Dir::B(c) => {
                if c < 3 && i + 1 < nx {
                    heap.push(Reverse(Node {
                        pos: (i + 1, j, Dir::B(c + 1)),
                        cost: node.cost + city[(i + 1, j)],
                        prev: Some((i, j, dir)),
                    }))
                }
                if j > 0 {
                    heap.push(Reverse(Node {
                        pos: (i, j - 1, Dir::L(1)),
                        cost: node.cost + city[(i, j - 1)],
                        prev: Some((i, j, dir)),
                    }))
                }
                if j + 1 < ny {
                    heap.push(Reverse(Node {
                        pos: (i, j + 1, Dir::R(1)),
                        cost: node.cost + city[(i, j + 1)],
                        prev: Some((i, j, dir)),
                    }))
                }
            }
            Dir::T(c) => {
                if c < 3 && i > 0 {
                    heap.push(Reverse(Node {
                        pos: (i - 1, j, Dir::T(c + 1)),
                        cost: node.cost + city[(i - 1, j)],
                        prev: Some((i, j, dir)),
                    }))
                }
                if j > 0 {
                    heap.push(Reverse(Node {
                        pos: (i, j - 1, Dir::L(1)),
                        cost: node.cost + city[(i, j - 1)],
                        prev: Some((i, j, dir)),
                    }))
                }
                if j + 1 < ny {
                    heap.push(Reverse(Node {
                        pos: (i, j + 1, Dir::R(1)),
                        cost: node.cost + city[(i, j + 1)],
                        prev: Some((i, j, dir)),
                    }))
                }
            }
            Dir::R(c) => {
                if c < 3 && j + 1 < ny {
                    heap.push(Reverse(Node {
                        pos: (i, j + 1, Dir::R(c + 1)),
                        cost: node.cost + city[(i, j + 1)],
                        prev: Some((i, j, dir)),
                    }))
                }
                if i > 0 {
                    heap.push(Reverse(Node {
                        pos: (i - 1, j, Dir::T(1)),
                        cost: node.cost + city[(i - 1, j)],
                        prev: Some((i, j, dir)),
                    }))
                }
                if i + 1 < nx {
                    heap.push(Reverse(Node {
                        pos: (i + 1, j, Dir::B(1)),
                        cost: node.cost + city[(i + 1, j)],
                        prev: Some((i, j, dir)),
                    }))
                }
            }
            Dir::L(c) => {
                if c < 3 && j > 0 {
                    heap.push(Reverse(Node {
                        pos: (i, j - 1, Dir::L(c + 1)),
                        cost: node.cost + city[(i, j - 1)],
                        prev: Some((i, j, dir)),
                    }))
                }
                if i > 0 {
                    heap.push(Reverse(Node {
                        pos: (i - 1, j, Dir::T(1)),
                        cost: node.cost + city[(i - 1, j)],
                        prev: Some((i, j, dir)),
                    }))
                }
                if i + 1 < nx {
                    heap.push(Reverse(Node {
                        pos: (i + 1, j, Dir::B(1)),
                        cost: node.cost + city[(i + 1, j)],
                        prev: Some((i, j, dir)),
                    }))
                }
            }
        }
    }
    panic!("no_path")
}

#[test]
fn test_process_p1() {
    assert_eq!(process_p1(TEST_CASE), 102)
}

fn process_p2(data: &str) -> usize {
    let city = process_pattern(data);
    let nx = city.shape()[0];
    let ny = city.shape()[1];
    let source = (0, 0);
    let target = (nx - 1, ny - 1);
    let mut explored = HashSet::new();
    let mut heap = BinaryHeap::from([
        Reverse(Node {
            pos: (source.0, source.1, Dir::R(0)),
            cost: 0,
            prev: None,
        }),
        Reverse(Node {
            pos: (source.0, source.1, Dir::B(0)),
            cost: 0,
            prev: None,
        }),
    ]);
    while let Some(Reverse(node)) = heap.pop() {
        if explored.contains(&node.pos) {
            continue;
        }
        explored.insert(node.pos);
        let (i, j, dir) = node.pos;
        if target == (i, j) && dir.unwrap_value() >= 4 {
            return node.cost;
        }
        match dir {
            Dir::B(c) => {
                if c < 10 && i + 1 < nx {
                    heap.push(Reverse(Node {
                        pos: (i + 1, j, Dir::B(c + 1)),
                        cost: node.cost + city[(i + 1, j)],
                        prev: Some((i, j, dir)),
                    }))
                }
                if c >= 4 && j > 0 {
                    heap.push(Reverse(Node {
                        pos: (i, j - 1, Dir::L(1)),
                        cost: node.cost + city[(i, j - 1)],
                        prev: Some((i, j, dir)),
                    }))
                }
                if c >= 4 && j + 1 < ny {
                    heap.push(Reverse(Node {
                        pos: (i, j + 1, Dir::R(1)),
                        cost: node.cost + city[(i, j + 1)],
                        prev: Some((i, j, dir)),
                    }))
                }
            }
            Dir::T(c) => {
                if c < 10 && i > 0 {
                    heap.push(Reverse(Node {
                        pos: (i - 1, j, Dir::T(c + 1)),
                        cost: node.cost + city[(i - 1, j)],
                        prev: Some((i, j, dir)),
                    }))
                }
                if c >= 4 && j > 0 {
                    heap.push(Reverse(Node {
                        pos: (i, j - 1, Dir::L(1)),
                        cost: node.cost + city[(i, j - 1)],
                        prev: Some((i, j, dir)),
                    }))
                }
                if c >= 4 && j + 1 < ny {
                    heap.push(Reverse(Node {
                        pos: (i, j + 1, Dir::R(1)),
                        cost: node.cost + city[(i, j + 1)],
                        prev: Some((i, j, dir)),
                    }))
                }
            }
            Dir::R(c) => {
                if c < 10 && j + 1 < ny {
                    heap.push(Reverse(Node {
                        pos: (i, j + 1, Dir::R(c + 1)),
                        cost: node.cost + city[(i, j + 1)],
                        prev: Some((i, j, dir)),
                    }))
                }
                if c >= 4 && i > 0 {
                    heap.push(Reverse(Node {
                        pos: (i - 1, j, Dir::T(1)),
                        cost: node.cost + city[(i - 1, j)],
                        prev: Some((i, j, dir)),
                    }))
                }
                if c >= 4 && i + 1 < nx {
                    heap.push(Reverse(Node {
                        pos: (i + 1, j, Dir::B(1)),
                        cost: node.cost + city[(i + 1, j)],
                        prev: Some((i, j, dir)),
                    }))
                }
            }
            Dir::L(c) => {
                if c < 10 && j > 0 {
                    heap.push(Reverse(Node {
                        pos: (i, j - 1, Dir::L(c + 1)),
                        cost: node.cost + city[(i, j - 1)],
                        prev: Some((i, j, dir)),
                    }))
                }
                if c >= 4 && i > 0 {
                    heap.push(Reverse(Node {
                        pos: (i - 1, j, Dir::T(1)),
                        cost: node.cost + city[(i - 1, j)],
                        prev: Some((i, j, dir)),
                    }))
                }
                if c >= 4 && i + 1 < nx {
                    heap.push(Reverse(Node {
                        pos: (i + 1, j, Dir::B(1)),
                        cost: node.cost + city[(i + 1, j)],
                        prev: Some((i, j, dir)),
                    }))
                }
            }
        }
    }
    panic!("no_path")
}

#[test]
fn test_process_p2() {
    assert_eq!(process_p2(TEST_CASE), 94)
}

fn main() {
    let data = std::fs::read_to_string("data/day17.txt").unwrap();
    let t0 = Instant::now();
    let result_p1 = process_p1(&data);
    let t1 = Instant::now();
    println!("The result of p1 is {}. ({:?})", result_p1, t1 - t0);
    let result_p2 = process_p2(&data);
    let t2 = Instant::now();
    println!("The result of p2 is {}. ({:?})", result_p2, t2 - t1);
}
