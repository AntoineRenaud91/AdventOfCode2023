use std::{path::Path, time::Instant, collections::{HashMap,VecDeque}, os::unix::process};

#[derive(Debug)]
enum Op<'a> {
    FlipFlop {modules: Vec<&'a str>,switch: bool},
    Conjonction {modules: Vec<&'a str>, memo: HashMap<&'a str,bool>}
}

fn process_p1(path: impl AsRef<Path>, n:usize) -> usize {
    let problem = std::fs::read_to_string(path).unwrap();
    let (mut map,init, all_mods) = problem.lines()
        .fold((HashMap::new(),VecDeque::new(),Vec::new()),|(mut map,mut init, mut all_mods ),line| {
            let (mut input_str,outputs_str) = line.split_once("->").unwrap();
            input_str = input_str.trim();
            let modules = outputs_str.split(',').map(|s| s.trim());
            match input_str.trim().chars().next().unwrap() {
                '%'=> {
                    all_mods.extend(modules.clone().map(|m| (m,&input_str[1..])));
                    map.insert(&input_str[1..], Op::FlipFlop{modules: modules.collect(), switch: false});
                },
                '&'=> {
                    all_mods.extend(modules.clone().map(|m| (m,&input_str[1..])));
                    map.insert(&input_str[1..], Op::Conjonction{modules: modules.collect(),memo: HashMap::new()});
                },
                _ => {
                    all_mods.extend(modules.clone().map(|m| (m,"init")));
                    init.extend(modules.map(|o| (o,false,"init")))
                },
            }
            (map,init,all_mods)
        });
    all_mods.into_iter().for_each(|(m,pred)| {
        if let Some(Op::Conjonction { modules:_, memo }) = map.get_mut(m) {
            memo.insert(pred, false);
        }
    });
    let mut low_pulse_count = 0;
    let mut high_pulse_count = 0;
    for _ in 0..n{
        low_pulse_count+=1;
        let mut pulses = init.clone();
        while let Some((module,pulse,pred))= pulses.pop_front() {
            if pulse {
                high_pulse_count+=1
            } else {
                low_pulse_count += 1
            };
            if let Some(op) = map.get_mut(module) {
                match op {
                    Op::FlipFlop{ modules,  switch} => {
                        if !pulse {
                            *switch= !*switch;
                            modules.iter().for_each(|m|  pulses.push_back((*m,*switch, module)));
                        }
                    },
                    Op::Conjonction{modules,memo} => {
                        *memo.get_mut(pred).unwrap()=pulse;
                        if memo.values().all(|v| *v) {
                            modules.iter().for_each(|m|  pulses.push_back((*m,false, module)));
                        } else {
                            modules.iter().for_each(|m|  pulses.push_back((*m,true, module)));
                        }
                    }
                }
            };
        }
    }
    high_pulse_count*low_pulse_count
}

#[test]
fn test_process_p1_e1() {
    let path = std::env::temp_dir().join("test_p1.dat");
    std::fs::write(&path, "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a").unwrap();
    assert_eq!(process_p1(path,10), 3200)
}

#[test]
fn test_process_p1_e2() {
    let path = std::env::temp_dir().join("test_p1.dat");
    std::fs::write(&path, "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output").unwrap();
    assert_eq!(process_p1(path,1000), 11687500)
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

fn lcm_of_iter<I: Iterator<Item = usize>>(numbers: I) -> usize {
    numbers.fold(1, |acc, num| {
        acc*(num / gcd(acc, num))
    })
}

fn process_p2(path: impl AsRef<Path>) -> usize {
    let problem = std::fs::read_to_string(path).unwrap();
    let graph = problem.lines().map(|line| {
        let (module, cables) = line.split_once(" -> ").unwrap();
        (module, cables.split(", ").collect::<Vec<_>>())
    } ).collect::<HashMap<_,_>>();
    let mut res = vec![];
    graph.get("broadcaster").unwrap().iter().for_each(|&m| {
        let mut flipflop = m;
        let mut bin = "".to_string();
        loop {
            let g = graph.get(format!("%{}", flipflop).as_str()).unwrap();
            bin = format!("{}{bin}",if g.len() == 2 || !graph.contains_key(format!("%{}",g[0]).as_str()) {1} else {0});
            let next_flipflops = g.iter().filter(|m| graph.contains_key(format!("%{}",m).as_str())).copied().collect::<Vec<_>>();
            if next_flipflops.is_empty() {
                break
            }
            flipflop = next_flipflops[0];
        }
        res.push(usize::from_str_radix(&bin, 2).unwrap())
    });
    lcm_of_iter(res.into_iter())
}

fn main() {
    let t0 = Instant::now();
    let result_p1 = process_p1("data/day20.txt",1000);
    let t1 = Instant::now();
    println!("The result of p1 is {}. ({:?})", result_p1, t1 - t0);
    let result_p2 = process_p2("data/day20.txt");
    let t2 = Instant::now();
    println!("The result of p2 is {}. ({:?})", result_p2, t2 - t1);
}
