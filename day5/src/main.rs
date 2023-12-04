use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn get_map(lines: &mut impl Iterator<Item = String>) -> Vec<[usize; 3]> {
    let mut range_map = vec![];
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        let mut nums = line.split(' ').map(|s| s.parse::<usize>().unwrap());
        let t_start = nums.next().unwrap();
        let s_start = nums.next().unwrap();
        let length = nums.next().unwrap();
        range_map.push([s_start, t_start, length])
    }
    lines.next();
    range_map
}

fn process_p1(path: impl AsRef<Path>) -> usize {
    let mut lines = BufReader::new(File::open(path).unwrap()).lines().flatten();
    let init_nums: Vec<usize> = lines
        .next()
        .unwrap()
        .split_once("seeds: ")
        .unwrap()
        .1
        .split(' ')
        .map(|s: &str| s.parse::<usize>().unwrap())
        .collect();
    lines.nth(1);
    (0..7)
        .fold(init_nums, |nums, _| {
            let map = get_map(&mut lines);
            nums.into_iter()
                .map(|i| {
                    for [s_start, t_start, length] in map.iter() {
                        if (i >= *s_start) && (i < s_start + length) {
                            return t_start + (i - s_start);
                        }
                    }
                    i
                })
                .collect()
        })
        .into_iter()
        .min()
        .unwrap()
}

#[test]
fn test_process_p1() {
    let path = std::env::temp_dir().join("test_p1.dat");
    std::fs::write(
        &path,
        "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
",
    )
    .unwrap();
    assert_eq!(process_p1(path), 35)
}

#[derive(Debug, PartialEq)]
struct MappedRange {
    mapped: Option<(usize, usize)>,
    unmapped: Vec<(usize, usize)>,
}

fn map_range(start: usize, length: usize, [s_start, t_start, mstep]: [usize; 3]) -> MappedRange {
    if (start >= s_start + mstep) || (start + length <= s_start) {
        return MappedRange {
            unmapped: vec![(start, length)],
            mapped: None,
        };
    }
    if (start >= s_start) && (start + length <= s_start + mstep) {
        return MappedRange {
            unmapped: vec![],
            mapped: Some((t_start + (start - s_start), length)),
        };
    }
    if (start < s_start) && (start + length > s_start + mstep) {
        return MappedRange {
            unmapped: vec![
                (start, s_start - start),
                (t_start + mstep, start + length - s_start - mstep),
            ],
            mapped: Some((t_start, mstep)),
        };
    }
    if start < s_start {
        return MappedRange {
            unmapped: vec![(start, s_start - start)],
            mapped: Some((t_start, start + length - s_start)),
        };
    }
    MappedRange {
        unmapped: vec![(s_start + mstep, start + length - s_start - mstep)],
        mapped: Some((t_start + (start - s_start), mstep + s_start - start)),
    }
}

#[test]
fn test_map_range() {
    assert_eq!(
        map_range(0, 5, [5, 5, 2]),
        MappedRange {
            unmapped: vec![(0, 5)],
            mapped: None
        }
    );
    assert_eq!(
        map_range(0, 6, [5, 5, 2]),
        MappedRange {
            unmapped: vec![(0, 5)],
            mapped: Some((5, 1))
        }
    );
    assert_eq!(
        map_range(0, 10, [5, 5, 2]),
        MappedRange {
            unmapped: vec![(0, 5), (7, 3)],
            mapped: Some((5, 2))
        }
    );
    assert_eq!(
        map_range(6, 4, [5, 5, 2]),
        MappedRange {
            unmapped: vec![(7, 3)],
            mapped: Some((6, 1))
        }
    )
}

fn process_p2(path: impl AsRef<Path>) -> usize {
    let mut lines = BufReader::new(File::open(path).unwrap()).lines().flatten();
    let init_nums: Vec<usize> = lines
        .next()
        .unwrap()
        .split_once("seeds: ")
        .unwrap()
        .1
        .split(' ')
        .map(|s: &str| s.parse::<usize>().unwrap())
        .collect();
    let init_nums: Vec<(usize, usize)> =
        init_nums.chunks(2).map(|nums| (nums[0], nums[1])).collect();
    lines.nth(1);
    (0..7)
        .fold(init_nums, |nums, _| {
            let map = get_map(&mut lines);
            nums.into_iter().fold(vec![], |mut r, (start, length)| {
                let mut rem = vec![(start, length)];
                for maprange in map.iter() {
                    rem = rem.into_iter().fold(vec![], |mut acc, (start, length)| {
                        let MappedRange { mapped, unmapped } = map_range(start, length, *maprange);
                        if let Some(v) = mapped {
                            r.push(v)
                        };
                        acc.extend(unmapped);
                        acc
                    });
                }
                r.extend(rem);
                r
            })
        })
        .into_iter()
        .map(|(i, _)| i)
        .min()
        .unwrap()
}

#[test]
fn test_process_p2() {
    let path = std::env::temp_dir().join("test_p2.dat");
    std::fs::write(
        &path,
        "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
",
    )
    .unwrap();
    assert_eq!(process_p2(path), 46)
}

fn main() {
    println!("The result of p1 is {}.", process_p1("data/day5.txt"));
    println!("The result of p2 is {}.", process_p2("data/day5.txt"));
}
