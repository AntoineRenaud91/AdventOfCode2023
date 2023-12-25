use std::time::Instant;

#[cfg(test)]
const TEST_CASE: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

fn process_p1(data: &str, [r, g, b]: [u32; 3]) -> u32 {
    data.lines().fold(0u32, |sum, line| {
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
    assert_eq!(process_p1(TEST_CASE, [12, 13, 14]), 8)
}

fn process_p2(data: &str) -> u32 {
    data.lines().fold(0u32, |sum, line| {
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
    assert_eq!(process_p2(TEST_CASE), 2286)
}

fn main() {
    let data = std::fs::read_to_string("data/day2.txt").unwrap();
    let t0 = Instant::now();
    let result_p1 = process_p1(&data, [12, 13, 14]);
    let t1 = Instant::now();
    println!("The result of p1 is {}. ({:?})", result_p1, t1 - t0);
    let result_p2 = process_p2(&data);
    let t2 = Instant::now();
    println!("The result of p2 is {}. ({:?})", result_p2, t2 - t1);
}
