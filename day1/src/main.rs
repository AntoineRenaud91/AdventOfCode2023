use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

const DIGITS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn process_p1(path: impl AsRef<Path>) -> u32 {
    BufReader::new(File::open(path).unwrap())
        .lines()
        .flatten()
        .fold(0u32, |acc, value| {
            let mut digits = value.chars().filter_map(|c| c.to_digit(10));
            let first_digit = digits.next().expect("No digit found");
            let last_digit = digits.last().unwrap_or(first_digit);
            acc + 10 * first_digit + last_digit
        })
}

#[test]
fn test_process_p1() {
    let path = std::env::temp_dir().join("testp1.dat");
    std::fs::write(
        &path,
        "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet",
    )
    .unwrap();
    assert_eq!(process_p1(path), 142)
}

fn process_p2(path: impl AsRef<Path>) -> u32 {
    BufReader::new(File::open(path).unwrap())
        .lines()
        .flatten()
        .fold(0u32, |acc, value| {
            let mut digits = value.chars().enumerate().filter_map(|(i, c)| {
                if let Some(n) = c.to_digit(10) {
                    return Some(n);
                }
                DIGITS.iter().enumerate().find_map(|(n, s)| {
                    if value[i..].starts_with(s) {
                        Some(n as u32)
                    } else {
                        None
                    }
                })
            });
            let first_digit = digits.next().expect("No digit found");
            let last_digit = digits.last().unwrap_or(first_digit);
            acc + 10 * first_digit + last_digit
        })
}

#[test]
fn test_process_p2() {
    let path = std::env::temp_dir().join("testp2.dat");
    std::fs::write(
        &path,
        "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen",
    )
    .unwrap();
    assert_eq!(process_p2(path), 281)
}

fn main() {
    println!("The result of p1 is {}.", process_p1("data/day1.txt"));
    println!("The result of p2 is {}.", process_p2("data/day1.txt"));
}
