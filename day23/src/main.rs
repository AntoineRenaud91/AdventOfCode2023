use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

use ndarray::{concatenate, Array2, Axis};

#[cfg(test)]
const TEST_CASE: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

fn process_pattern(data: &str) -> Array2<char> {
    let ncols = data.lines().next().unwrap().len();
    data.lines().filter(|line| !line.is_empty()).fold(
        Array2::from_shape_vec([0, ncols], vec![]).unwrap(),
        |pattern, line| {
            concatenate![
                Axis(0),
                pattern.view(),
                Array2::from_shape_vec([1, ncols], line.chars().collect())
                    .unwrap()
                    .view()
            ]
        },
    )
}

fn longuest_path_p1(
    grid: &Array2<char>,
    mut path: HashSet<(usize, usize)>,
    (i, j): (usize, usize),
    target: (usize, usize),
) -> usize {
    path.insert((i, j));
    if (i, j) == target {
        return path.len();
    }
    match grid[(i, j)] {
        '^' => {
            if path.contains(&(i - 1, j)) {
                0
            } else {
                longuest_path_p1(grid, path, (i - 1, j), target)
            }
        }
        '>' => {
            if path.contains(&(i, j + 1)) {
                0
            } else {
                longuest_path_p1(grid, path, (i, j + 1), target)
            }
        }
        '<' => {
            if path.contains(&(i, j - 1)) {
                0
            } else {
                longuest_path_p1(grid, path, (i, j - 1), target)
            }
        }
        'v' => {
            if path.contains(&(i + 1, j)) {
                0
            } else {
                longuest_path_p1(grid, path, (i + 1, j), target)
            }
        }
        '.' => [
            Some((i + 1, j)),
            Some((i, j + 1)),
            i.checked_sub(1).map(|i| (i, j)),
            j.checked_sub(1).map(|j| (i, j)),
        ]
        .into_iter()
        .flatten()
        .filter(|node| !path.contains(node))
        .filter(|node| grid.get(*node).unwrap_or(&'#') != &'#')
        .map(|node| longuest_path_p1(grid, path.clone(), node, target))
        .max()
        .unwrap_or(0),
        _ => {
            panic!("here")
        }
    }
}

fn process_p1(data: &str) -> usize {
    let grid = process_pattern(data);
    let source = (0, 1);
    let target = (grid.shape()[0] - 1, grid.shape()[1] - 2);
    longuest_path_p1(&grid, HashSet::new(), source, target) - 1
}

#[test]
fn test_process_p1() {
    assert_eq!(process_p1(TEST_CASE), 94)
}

fn find_forks(grid: &Array2<char>) -> impl Iterator<Item = (usize, usize)> + '_ {
    (1..grid.shape()[0] - 1)
        .flat_map(|i| (1..grid.shape()[1] - 1).map(move |j| (i, j)))
        .filter(|&ind| grid[ind] != '#')
        .filter(|&(i, j)| {
            [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)]
                .into_iter()
                .filter(|&node| grid[node] != '#')
                .count()
                > 2
        })
}

type Graph = HashMap<usize, HashSet<(usize, usize)>>;

fn build_graph(grid: &Array2<char>, nodes: &HashMap<(usize, usize), usize>) -> Graph {
    let mut graph: Graph = HashMap::new();
    let mut explored = HashSet::new();
    let mut heap = vec![((0, 1), None, 0)];
    while let Some(((i, j), mut pred, mut steps)) = heap.pop() {
        if let Some(ind) = nodes.get(&(i, j)) {
            if let Some(pred) = pred {
                if let Some(set) = graph.get_mut(ind) {
                    set.insert((pred, steps));
                } else {
                    graph.insert(*ind, HashSet::from([(pred, steps)]));
                }
                if let Some(set) = graph.get_mut(&pred) {
                    set.insert((*ind, steps));
                } else {
                    graph.insert(pred, HashSet::from([(*ind, steps)]));
                }
            }
            steps = 0;
            pred = Some(*ind);
        };
        if explored.contains(&(i, j)) {
            continue;
        }
        explored.insert((i, j));
        [
            Some((i + 1, j)),
            Some((i, j + 1)),
            i.checked_sub(1).map(|i| (i, j)),
            j.checked_sub(1).map(|j| (i, j)),
        ]
        .into_iter()
        .flatten()
        .filter(|node| grid.get(*node).unwrap_or(&'#') != &'#')
        .for_each(|next| heap.push((next, pred, steps + 1)))
    }
    graph
}

fn longuest_path_p2(
    graph: &Graph,
    mut path: HashMap<usize, usize>,
    (node, cost): (usize, usize),
    target: usize,
) -> usize {
    path.insert(node, cost);
    if node == target {
        return path.values().sum();
    }
    graph
        .get(&node)
        .unwrap()
        .iter()
        .filter(|(node, _)| !path.contains_key(node))
        .map(|&(node, cost)| longuest_path_p2(graph, path.clone(), (node, cost), target))
        .max()
        .unwrap_or(0)
}

fn process_p2(data: &str) -> usize {
    let grid = process_pattern(data);
    let source = (0, 1);
    let target = (grid.shape()[0] - 1, grid.shape()[1] - 2);
    let nodes = HashMap::from_iter(
        [(source, 0), (target, 1)]
            .into_iter()
            .chain(find_forks(&grid).enumerate().map(|(i, n)| (n, i + 2))),
    );
    let graph = build_graph(&grid, &nodes);
    longuest_path_p2(&graph, HashMap::new(), (0, 0), 1)
}

#[test]
fn test_process_p2() {
    assert_eq!(process_p2(TEST_CASE), 154)
}

fn main() {
    let data = std::fs::read_to_string("data/day23.txt").unwrap();
    let t0 = Instant::now();
    let result_p1 = process_p1(&data);
    let t1 = Instant::now();
    println!("The result of p1 is {}. ({:?})", result_p1, t1 - t0);
    let result_p2 = process_p2(&data);
    let t2 = Instant::now();
    println!("The result of p2 is {}. ({:?})", result_p2, t2 - t1);
}
