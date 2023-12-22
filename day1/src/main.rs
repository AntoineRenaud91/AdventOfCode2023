use std::time::Instant;

const DIGITS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn process_p1(data: &str) -> u32 {
    data.lines().fold(0u32, |acc, value| {
        let mut digits = value.chars().filter_map(|c| c.to_digit(10));
        let first_digit = digits.next().expect("No digit found");
        let last_digit = digits.last().unwrap_or(first_digit);
        acc + 10 * first_digit + last_digit
    })
}

#[test]
fn test_process_p1() {
    assert_eq!(
        process_p1(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
        ),
        142
    )
}

fn process_p2(data: &str) -> u32 {
    data.lines().fold(0u32, |acc, value| {
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
    assert_eq!(
        process_p2(
            "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen"
        ),
        281
    )
}

fn main() {
    let t0 = Instant::now();
    let result_p1 = process_p1(&std::fs::read_to_string("data/day1.txt").unwrap());
    let t1 = Instant::now();
    let result_p2 = process_p2(&std::fs::read_to_string("data/day1.txt").unwrap());
    let t2 = Instant::now();
    println!("The result of p1 is {}. ({:?})", result_p1, t1 - t0);
    println!("The result of p2 is {}. ({:?})", result_p2, t2 - t1);
}
