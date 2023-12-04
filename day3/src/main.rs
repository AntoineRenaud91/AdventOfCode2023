use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn process_p1(path: impl AsRef<Path>) -> u32 {
    let mut result = 0u32;
    let mut lines = BufReader::new(File::open(path).unwrap()).lines().flatten();
    let mut prev_line: Option<String> = None;
    let mut cur_line = lines.next();
    let mut next_line = lines.next();
    while let Some(s) = cur_line.as_ref() {
        result = s
            .split(|c: char| !c.is_ascii_digit())
            .fold((0, result), |(index, value), digit| {
                if digit.is_empty() {
                    return (index + 1, value);
                }
                let num = digit.parse::<u32>().unwrap();
                let next_index = index + digit.len();
                let min_index = if index > 0 {
                    if s.chars().nth(index - 1).unwrap() != '.' {
                        return (next_index + 1, value + num);
                    }
                    index - 1
                } else {
                    index
                };
                let max_index = if next_index == s.len() {
                    next_index - 1
                } else {
                    if s.chars().nth(next_index).unwrap() != '.' {
                        return (next_index + 1, value + num);
                    }
                    next_index
                };
                for i in min_index..(max_index + 1) {
                    if prev_line
                        .as_ref()
                        .map(|line| line.chars().nth(i).unwrap() != '.')
                        .unwrap_or(false)
                        | next_line
                            .as_ref()
                            .map(|line| line.chars().nth(i).unwrap() != '.')
                            .unwrap_or(false)
                    {
                        return (next_index + 1, value + num);
                    }
                }
                (next_index + 1, value)
            })
            .1;
        prev_line = cur_line;
        cur_line = next_line;
        next_line = lines.next();
    }
    result
}

#[test]
fn test_process_p1() {
    let path = std::env::temp_dir().join("test_p1.dat");
    std::fs::write(
        &path,
        "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
    )
    .unwrap();
    assert_eq!(process_p1(path), 4361)
}

fn process_p2(path: impl AsRef<Path>) -> u32 {
    let mut lines = BufReader::new(File::open(path).unwrap()).lines().flatten();
    let mut prev_line: Option<String> = None;
    let mut cur_line = lines.next();
    let mut next_line = lines.next();
    let mut row = 0;
    let mut gears: HashMap<(i32, usize), Vec<u32>> = HashMap::new();
    while let Some(s) = cur_line.as_ref() {
        let mut col = 0;
        s.split(|c: char| !c.is_ascii_digit()).for_each(|digit| {
            if digit.is_empty() {
                col += 1;
                return;
            }
            let num = digit.parse::<u32>().unwrap();
            let next_col = col + digit.len();
            let min_col = if col > 0 {
                if s.chars().nth(col - 1).unwrap() == '*' {
                    if let Some(nums) = gears.get_mut(&(row, col - 1)) {
                        nums.push(num)
                    } else {
                        gears.insert((row, col - 1), vec![num]);
                    }
                }
                col - 1
            } else {
                col
            };
            let max_col = if next_col == s.len() {
                next_col - 1
            } else {
                if s.chars().nth(next_col).unwrap() == '*' {
                    if let Some(nums) = gears.get_mut(&(row, next_col)) {
                        nums.push(num)
                    } else {
                        gears.insert((row, next_col), vec![num]);
                    }
                }
                next_col
            };
            for i in min_col..(max_col + 1) {
                if prev_line
                    .as_ref()
                    .map(|line| line.chars().nth(i).unwrap() == '*')
                    .unwrap_or(false)
                {
                    if let Some(nums) = gears.get_mut(&(row - 1, i)) {
                        nums.push(num);
                    } else {
                        gears.insert((row - 1, i), vec![num]);
                    }
                }
                if next_line
                    .as_ref()
                    .map(|line| line.chars().nth(i).unwrap() == '*')
                    .unwrap_or(false)
                {
                    if let Some(nums) = gears.get_mut(&(row + 1, i)) {
                        nums.push(num);
                    } else {
                        gears.insert((row + 1, i), vec![num]);
                    }
                }
            }
            col = next_col + 1
        });
        row += 1;
        prev_line = cur_line;
        cur_line = next_line;
        next_line = lines.next();
    }
    gears
        .values()
        .filter_map(|v| {
            if v.len() == 2 {
                Some(v.iter().product::<u32>())
            } else {
                None
            }
        })
        .sum::<u32>()
}

#[test]
fn test_process_p2() {
    let path = std::env::temp_dir().join("test_p2.dat");
    std::fs::write(
        &path,
        "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
..........
....*10...
..10......",
    )
    .unwrap();
    assert_eq!(process_p2(path), 467935)
}

fn main() {
    println!("The result of p1 is {}.", process_p1("data/day3.txt"));
    println!("The result of p2 is {}.", process_p2("data/day3.txt"));
}
