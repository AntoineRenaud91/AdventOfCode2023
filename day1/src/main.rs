use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

const DIGITS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn sum_values_digit_only(path: impl AsRef<Path>) -> u32 {
    BufReader::new(File::open(path).unwrap())
        .lines()
        .flatten()
        .fold(0u32, |acc, value| {
            let first_digit = value.chars().find_map(|c| c.to_digit(10));
            let last_digit = value.chars().rev().find_map(|c| c.to_digit(10));
            let number = match (first_digit, last_digit) {
                (Some(tens), Some(units)) => 10 * tens + units,
                _ => {
                    println!("Error occured with value: {value}");
                    0
                }
            };
            number + acc
        })
}

fn sum_values_all(path: impl AsRef<Path>) -> u32 {
    BufReader::new(File::open(path).unwrap())
        .lines()
        .flatten()
        .fold(0u32, |acc, value| {
            let first_digit = value.chars().enumerate().find_map(|(i, c)| {
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
            let last_digit = value.chars().rev().enumerate().find_map(|(i, c)| {
                if let Some(n) = c.to_digit(10) {
                    return Some(n);
                }
                DIGITS.iter().enumerate().find_map(|(n, s)| {
                    if value[..(value.len() - i)].ends_with(s) {
                        Some(n as u32)
                    } else {
                        None
                    }
                })
            });
            let number = match (first_digit, last_digit) {
                (Some(tens), Some(units)) => 10 * tens + units,
                _ => {
                    println!("Error occured with value: {value}");
                    0
                }
            };
            number + acc
        })
}

fn main() {
    println!(
        "The result with only digits is {}!",
        sum_values_digit_only("data/calibration_values.data")
    );
    println!(
        "The corrected result is {}!",
        sum_values_all("data/calibration_values.data")
    );
}
