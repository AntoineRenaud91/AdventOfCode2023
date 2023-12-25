use std::time::Instant;

#[cfg(test)]
const TEST_CASE: &str = "seeds: 79 14 55 13

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
56 93 4";

fn parse_line_to_usize(line: &str) -> Vec<usize> {
    line.split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn get_ranges<'a, I: Iterator<Item = &'a str>>(lines: &mut I) -> Vec<[usize; 3]> {
    lines
        .skip(1)
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let nums = parse_line_to_usize(line);
            [nums[1], nums[0], nums[2]]
        })
        .collect()
}

fn process_p1(data: &str) -> usize {
    let mut lines = data.lines();
    let initial_values = lines
        .next()
        .as_ref()
        .and_then(|line| line.split_once("seeds: "))
        .map(|(_, values)| parse_line_to_usize(values))
        .expect("Ill defined seeds"); // process initial values
    lines.next(); // skip a line
    (0..7)
        .fold(initial_values, |nums, _| {
            let ranges = get_ranges(&mut lines);
            nums.into_iter()
                .map(|i| {
                    ranges
                        .iter()
                        .find_map(|&[s_start, t_start, length]| {
                            if i >= s_start && i < s_start + length {
                                Some(t_start + (i - s_start))
                            } else {
                                None
                            }
                        })
                        .unwrap_or(i)
                })
                .collect()
        })
        .into_iter()
        .min()
        .expect("No seed number")
}

#[test]
fn test_process_p1() {
    assert_eq!(process_p1(TEST_CASE), 35)
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

fn process_p2(data: &str) -> usize {
    let mut lines = data.lines();
    let initial_values = lines
        .next()
        .as_ref()
        .and_then(|line| line.split_once("seeds: "))
        .map(|(_, values)| {
            parse_line_to_usize(values)
                .chunks(2)
                .map(|nums| (nums[0], nums[1]))
                .collect::<Vec<_>>()
        })
        .expect("Ill defined seeds");
    lines.next(); //skip a line
    (0..7)
        .fold(initial_values, |nums, _| {
            let map = get_ranges(&mut lines);
            nums.into_iter()
                .fold(vec![], |mut result, (start, length)| {
                    let mut remaining = vec![(start, length)];
                    for &range in &map {
                        remaining =
                            remaining
                                .into_iter()
                                .fold(vec![], |mut acc, (start, length)| {
                                    let MappedRange { mapped, unmapped } =
                                        map_range(start, length, range);
                                    if let Some(mapped_value) = mapped {
                                        result.push(mapped_value);
                                    }
                                    acc.extend(unmapped);
                                    acc
                                });
                    }
                    result.extend(remaining);
                    result
                })
        })
        .into_iter()
        .map(|(i, _)| i)
        .min()
        .unwrap()
}

#[test]
fn test_process_p2() {
    assert_eq!(process_p2(TEST_CASE), 46)
}

fn main() {
    let data = std::fs::read_to_string("data/day5.txt").unwrap();
    let t0 = Instant::now();
    let result_p1 = process_p1(&data);
    let t1 = Instant::now();
    println!("The result of p1 is {}. ({:?})", result_p1, t1 - t0);
    let result_p2 = process_p2(&data);
    let t2 = Instant::now();
    println!("The result of p2 is {}. ({:?})", result_p2, t2 - t1);
}
