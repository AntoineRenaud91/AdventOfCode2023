use std::{path::Path, time::Instant};

fn winable_config(t_race: usize, d_record: usize) -> usize {
    let x = (t_race as f64) / 2.0;
    let mut x = (x - (x.powi(2) - d_record as f64).sqrt()) as usize;
    if x * (t_race - x) > d_record {
        loop {
            x -= 1;
            if x * (t_race - x) <= d_record {
                return t_race - 2 * x - 1;
            }
        }
    }
    loop {
        x += 1;
        if x * (t_race - x) > d_record {
            return t_race - 2 * (x - 1) - 1;
        };
    }
}

#[test]
fn test_winable_config() {
    let t_races: [usize; 3] = [7, 15, 30];
    let d_records: [usize; 3] = [9, 40, 200];
    let results: [usize; 3] = [4, 8, 9];
    for ((t_race, d_record), r) in t_races.into_iter().zip(d_records).zip(results) {
        assert_eq!(winable_config(t_race, d_record), r)
    }
}

fn process_p1(path: impl AsRef<Path>) -> usize {
    let input = std::fs::read_to_string(path).unwrap();
    let (race_times, record_distances) = input.split_once('\n').unwrap();
    let race_times = race_times
        .split_at(11)
        .1
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap());
    let record_distances = record_distances
        .split_at(11)
        .1
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap());
    race_times
        .zip(record_distances)
        .fold(1usize, |result, (t_race, d_record)| {
            winable_config(t_race, d_record) * result
        })
}

#[test]
fn test_process_p1() {
    let path = std::env::temp_dir().join("test_p1.dat");
    std::fs::write(
        &path,
        "Time:      7  15   30
Distance:  9  40  200",
    )
    .unwrap();
    assert_eq!(process_p1(path), 288)
}

fn process_p2(path: impl AsRef<Path>) -> usize {
    let input = std::fs::read_to_string(path).unwrap();
    let (t_race, d_record) = input.split_once('\n').unwrap();
    let t_race = t_race
        .split_at(11)
        .1
        .replace(' ', "")
        .parse::<usize>()
        .unwrap();
    let d_record = d_record
        .split_at(11)
        .1
        .replace(' ', "")
        .parse::<usize>()
        .unwrap();
    winable_config(t_race, d_record)
}

#[test]
fn test_process_p2() {
    let path = std::env::temp_dir().join("test_p2.dat");
    std::fs::write(
        &path,
        "Time:      7  15   30
Distance:  9  40  200",
    )
    .unwrap();
    assert_eq!(process_p2(path), 71503)
}

fn main() {
    let t0 = Instant::now();
    let result_p1 = process_p1("data/day6.txt");
    let t1 = Instant::now();
    let result_p2 = process_p2("data/day6.txt");
    let t2 = Instant::now();
    println!("The result of p1 is {}. ({:?})", result_p1, t1 - t0);
    println!("The result of p2 is {}. ({:?})", result_p2, t2 - t1);
}
