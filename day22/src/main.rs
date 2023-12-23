use std::{
    cmp::Ordering,
    collections::{BTreeMap, HashMap, HashSet},
    time::Instant,
};

#[derive(Debug)]
enum Dir {
    X(usize),
    Y(usize),
    Z,
}

#[derive(Debug)]
struct Brick {
    start: [usize; 3],
    dir: Dir,
    vlen: usize,
}

fn get_position(data: &str) -> [usize; 3] {
    let mut data_iter = data.split(',').flat_map(|s| s.parse::<usize>());
    [
        data_iter.next().unwrap(),
        data_iter.next().unwrap(),
        data_iter.next().unwrap(),
    ]
}

impl Brick {
    fn new(data: &str) -> Self {
        let (start_str, end_str) = data.split_once('~').unwrap();
        let start = get_position(start_str);
        let end = get_position(end_str);
        if start[0] < end[0] {
            Self {
                start,
                dir: Dir::X(end[0] - start[0] + 1),
                vlen: 0,
            }
        } else if start[1] < end[1] {
            Self {
                start,
                dir: Dir::Y(end[1] - start[1] + 1),
                vlen: 0,
            }
        } else {
            Self {
                start,
                dir: Dir::Z,
                vlen: end[2] - start[2],
            }
        }
    }

    fn h_pos_range(&self) -> Box<dyn Iterator<Item = [usize; 2]> + '_> {
        match self.dir {
            Dir::X(size) => Box::new((0..size).map(|k| [self.start[0] + k, self.start[1]])),
            Dir::Y(size) => Box::new((0..size).map(|k| [self.start[0], self.start[1] + k])),
            Dir::Z => Box::new([[self.start[0], self.start[1]]].into_iter()),
        }
    }
}

fn process_p1(data: &str) -> usize {
    let mut bricks = data.lines().map(Brick::new).collect::<Vec<_>>();
    bricks.sort_by_key(|brick| brick.start[2]);
    let mut level_map: BTreeMap<[usize; 2], (usize, usize)> = BTreeMap::new();
    let mut tree = HashMap::new();
    for (i, brick) in bricks.iter_mut().enumerate() {
        let (level, supported_by) =
            brick
                .h_pos_range()
                .fold((0, HashSet::new()), |(mut level, mut supported_by), pos| {
                    if let Some((z, s)) = level_map.get(&pos) {
                        match z.cmp(&level) {
                            Ordering::Equal => {
                                supported_by.insert(*s);
                            }
                            Ordering::Greater => {
                                level = *z;
                                supported_by = HashSet::from([*s]);
                            }
                            Ordering::Less => {}
                        }
                    }
                    (level, supported_by)
                });
        brick.start[2] = level + 1;
        brick.h_pos_range().for_each(|pos| {
            level_map.insert(pos, (level + 1 + brick.vlen, i));
        });
        tree.insert(i, supported_by);
    }
    tree.retain(|_, set| !set.is_empty());
    let rev_tree = tree.iter().fold(
        HashMap::<usize, HashSet<usize>>::new(),
        |mut tree, (k, set)| {
            set.iter().for_each(|l| {
                if let Some(set) = tree.get_mut(l) {
                    set.insert(*k);
                } else {
                    tree.insert(*l, HashSet::from([*k]));
                }
            });
            tree
        },
    );
    (0..bricks.len())
        .filter(|i| {
            if let Some(set) = rev_tree.get(i) {
                set.iter().all(|j| tree.get(j).unwrap().len() > 1)
            } else {
                true
            }
        })
        .count()
}

#[test]
fn test_process_p1() {
    assert_eq!(
        process_p1(
            "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9"
        ),
        5
    )
}

fn count_fall(
    tree: &HashMap<usize, HashSet<usize>>,
    rev_tree: &HashMap<usize, HashSet<usize>>,
    mut fallen: HashSet<usize>,
    just_fallen: HashSet<usize>,
) -> usize {
    fallen.extend(just_fallen.clone());
    let just_fallen = just_fallen
        .iter()
        .fold(HashSet::new(), |mut just_fallen, index| {
            if let Some(set) = rev_tree.get(index) {
                for j in set {
                    if tree.get(j).unwrap().iter().all(|k| fallen.contains(k)) {
                        just_fallen.insert(*j);
                    }
                }
            };
            just_fallen
        });
    if just_fallen.is_empty() {
        fallen.len() - 1
    } else {
        count_fall(tree, rev_tree, fallen, just_fallen)
    }
}

#[test]
fn test_count_fall() {
    let rev_tree = HashMap::from([
        (0, HashSet::from([1, 2])),
        (1, HashSet::from([3])),
        (2, HashSet::from([3])),
        (1, HashSet::from([3])),
        (3, HashSet::from([4])),
        (4, HashSet::from([5])),
    ]);
    let tree = HashMap::from([
        (1, HashSet::from([0])),
        (2, HashSet::from([0])),
        (3, HashSet::from([1, 2])),
        (4, HashSet::from([3])),
        (5, HashSet::from([4])),
    ]);
    assert_eq!(
        count_fall(&tree, &rev_tree, HashSet::new(), HashSet::from([0])),
        5
    )
}

fn process_p2(data: &str) -> usize {
    let mut bricks = data.lines().map(Brick::new).collect::<Vec<_>>();
    bricks.sort_by_key(|brick| brick.start[2]);
    let mut level_map: BTreeMap<[usize; 2], (usize, usize)> = BTreeMap::new();
    let mut tree = HashMap::new();
    for (i, brick) in bricks.iter_mut().enumerate() {
        let (level, supported_by) =
            brick
                .h_pos_range()
                .fold((0, HashSet::new()), |(mut level, mut supported_by), pos| {
                    if let Some((z, s)) = level_map.get(&pos) {
                        match z.cmp(&level) {
                            Ordering::Equal => {
                                supported_by.insert(*s);
                            }
                            Ordering::Greater => {
                                level = *z;
                                supported_by = HashSet::from([*s]);
                            }
                            Ordering::Less => {}
                        }
                    }
                    (level, supported_by)
                });
        brick.start[2] = level + 1;
        brick.h_pos_range().for_each(|pos| {
            level_map.insert(pos, (level + 1 + brick.vlen, i));
        });
        tree.insert(i, supported_by);
    }
    tree.retain(|_, set| !set.is_empty());
    let rev_tree = tree.iter().fold(
        HashMap::<usize, HashSet<usize>>::new(),
        |mut tree, (k, set)| {
            set.iter().for_each(|l| {
                if let Some(set) = tree.get_mut(l) {
                    set.insert(*k);
                } else {
                    tree.insert(*l, HashSet::from([*k]));
                }
            });
            tree
        },
    );
    (0..bricks.len())
        .map(|i| count_fall(&tree, &rev_tree, HashSet::new(), HashSet::from([i])))
        .sum()
}

#[test]
fn test_process_p2() {
    assert_eq!(
        process_p2(
            "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9"
        ),
        7
    )
}

fn main() {
    let t0 = Instant::now();
    let result_p1 = process_p1(&std::fs::read_to_string("data/day22.txt").unwrap());
    let t1 = Instant::now();
    println!("The result of p1 is {}. ({:?})", result_p1, t1 - t0);
    let result_p2 = process_p2(&std::fs::read_to_string("data/day22.txt").unwrap());
    let t2 = Instant::now();
    println!("The result of p2 is {}. ({:?})", result_p2, t2 - t1);
}
