use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn process_p1(path: impl AsRef<Path>, [r, g, b]: [u32; 3]) -> u32 {
    BufReader::new(File::open(path).unwrap())
        .lines()
        .flatten()
        .fold(0u32, |sum, line| {
            let (game, sets) = line.split_once(':').expect("':' not  in line");
            let valid = sets.split(';').all(|set| {
                for s in set.split(',') {
                    let (num, color) = s[1..]
                        .split_once(' ')
                        .expect("' ' not in 'num color' pattern");
                    let num = num.parse::<u32>().expect("num is not a u32");
                    if match color {
                        "red" => r,
                        "green" => g,
                        "blue" => b,
                        _ => panic!("{color} not in [green,red,blue]"),
                    } < num
                    {
                        return false;
                    }
                }
                true
            });
            if valid {
                sum + game[5..].parse::<u32>().expect("Id is not a u32")
            } else {
                sum
            }
        })
}

#[test]
fn test_process_p1() {
    let path = std::env::temp_dir().join("test_p1.dat");
    std::fs::write(
        &path,
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
    )
    .unwrap();
    assert_eq!(process_p1(path, [12, 13, 14]), 8)
}

fn process_p2(path: impl AsRef<Path>) -> u32 {
    BufReader::new(File::open(path).unwrap())
        .lines()
        .flatten()
        .fold(0u32, |sum, line| {
            let (_, sets) = line.split_once(':').expect("':' not  in line");
            let [r, g, b] = sets
                .split(';')
                .fold([0u32; 3], |[mut r, mut g, mut b], set| {
                    for s in set.split(',') {
                        let (num, color) = s[1..]
                            .split_once(' ')
                            .expect("' ' not in 'num color' pattern");
                        let num = num.parse::<u32>().expect("num is not a u32");
                        match color {
                            "red" => {
                                if num > r {
                                    r = num
                                }
                            }
                            "green" => {
                                if num > g {
                                    g = num
                                }
                            }
                            "blue" => {
                                if num > b {
                                    b = num
                                }
                            }
                            _ => panic!("{color} not in [green,red,blue]"),
                        };
                    }
                    [r, g, b]
                });
            sum + r * g * b
        })
}

#[test]
fn test_process_p2() {
    let path = std::env::temp_dir().join("test_p1.dat");
    std::fs::write(
        &path,
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
    )
    .unwrap();
    assert_eq!(process_p2(path), 2286)
}

fn main() {
    println!(
        "The result of p1 is {}.",
        process_p1("data/day2.txt", [12, 13, 14])
    );
    println!("The result of p2 is {}.", process_p2("data/day2.txt"));
}
