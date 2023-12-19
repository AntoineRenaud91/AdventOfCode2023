use std::{path::Path, time::Instant, cmp::Ordering, collections::HashMap};

#[derive(Debug)]
enum Part {
    X,M,A,S
}

impl Part {
    fn get(&self, parts: &Parts) -> usize {
        match self {
            Self::X => parts.x,
            Self::M => parts.m,
            Self::A => parts.a,
            Self::S => parts.s
        }
    }
}

impl From<char> for Part {
    fn from(value: char) -> Self {
        match value {
            'x' => Self::X,
            'm' => Self::M,
            'a' => Self::A,
            's' => Self::S,
            _ => panic!("")
        }
    }
}

#[derive(Debug,Clone,Copy)]
enum Dest<'a> {
    Next(&'a str), Stop(bool)
}

impl<'a> From<&'a str> for Dest<'a> {
    fn from(value: &'a str) -> Self {
        match value {
            "R" => Self::Stop(false),
            "A" => Self::Stop(true),
            _ => Self::Next(value)
        }
    }
}


#[derive(Debug)]
struct Rule<'a> {
    part: Part,
    op: Ordering,
    value: usize,
    dest: Dest<'a>
}

fn split_range((i,j): (usize,usize),op: Ordering, value: usize) -> (Option<(usize,usize)>,Option<(usize,usize)>) {
    match op {
        Ordering::Less =>  if j<value { (Some((i,j)),None)} 
        else if i>=value {(None, Some((i,j)))} else {
            (Some((i,value-1)),Some((value,j)))
        },
        _=> if i>value { (Some((i,j)),None)} 
        else if j<=value {(None, Some((i,j)))} else {
            (Some((value+1,j)),Some((i,value)))
        }
    }
}

#[test]
fn test_split_range(){
    assert_eq!(split_range((2001,4000), Ordering::Greater, 3000),(Some((3001,4000)),Some((2001,3000))))
}

impl<'a> Rule<'a> {
    fn check(&self, parts:&Parts) -> Option<Dest<'a>> {
        if self.part.get(parts).cmp(&self.value) == self.op {
            Some(self.dest)
        } else {None}
    }
    fn split_partsrange(&self, partsrange: PartsRange) -> (Option<PartsRange>,Option<PartsRange>) {
        let PartsRange { x, m, a, s } = partsrange;
        match self.part {
            Part::X => {
                let (x1,x2) = split_range(x, self.op, self.value);
                (x1.map(|x| PartsRange{x,m,a,s}),x2.map(|x| PartsRange { x, m, a, s }))
            },
            Part::M => {
                let (m1,m2) = split_range(m, self.op, self.value);
                (m1.map(|m| PartsRange{x,m,a,s}),m2.map(|m| PartsRange { x, m, a, s }))                
            },
            Part::A => {
                let (a1,a2) = split_range(a, self.op, self.value);
                (a1.map(|a| PartsRange{x,m,a,s}),a2.map(|a| PartsRange { x, m, a, s }))

            },
            Part::S => {
                let (s1,s2) = split_range(s, self.op, self.value);
                (s1.map(|s| PartsRange{x,m,a,s}),s2.map(|s| PartsRange { x, m, a, s }))
            }
         }
    }
}

#[derive(Debug)]
struct Parts {
    a: usize,
    x: usize,
    m: usize,
    s: usize
}

fn process_parts(ruleset: &HashMap<&str,(Vec<Rule<'_>>,Dest<'_>)>, parts: &Parts, dest: Dest<'_>) -> bool {
    let (rules,dest) = match dest {
        Dest::Stop(c) => return c,
        Dest::Next(name) => ruleset.get(name).unwrap()
    };
    for rule in rules {
        if let Some(dest) = rule.check(parts) {
            return process_parts(ruleset, parts, dest)
        }
    };
    process_parts(ruleset, parts, *dest)
}

fn process_p1(path: impl AsRef<Path>) -> usize {
    let problem = std::fs::read_to_string(path).unwrap();
    let (ruleset,parts) = problem.split_once("\n\n").unwrap();
    let ruleset = ruleset.lines().map(|line| {
        let (name, rem) = line.split_once('{').unwrap();
        let mut rules = rem.trim_end_matches('}')
            .split(',');
        let stop: Dest = rules.next_back().unwrap().into();
        let rules = rules.map(|s| {
                let (cond, dest) = s.split_once(':').unwrap();
                let value = cond[2..].parse::<usize>().unwrap();
                let part = s.chars().next().unwrap().into();
                let op = match s.chars().nth(1).unwrap() {
                    '>'=> Ordering::Greater,
                    '<'=> Ordering::Less,
                    _ => panic!("")
                };
                Rule {part,op,value,dest:dest.into()}
            }).collect::<Vec<_>>();
        (name,(rules,stop))
    }).collect::<HashMap<_,_>>();
    parts.lines().map(|line| 
            line[1..line.len()-1].split(',')
                .map(|s| s.split_once('=').unwrap().1.parse::<usize>().unwrap()).collect::<Vec<_>>())
                .map(|p| Parts {x:p[0],m: p[1], a: p[2], s: p[3]})
                .filter(|p| {
                    process_parts(&ruleset, p, Dest::Next("in"))
                })
                .map(|p| p.x+p.m+p.a+p.s).sum()
}

#[test]
fn test_process_p1() {
    let path = std::env::temp_dir().join("test_p1.dat");
    std::fs::write(&path, "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}").unwrap();
    assert_eq!(process_p1(path), 19114)
}

#[derive(Debug,Clone,Copy)]
struct PartsRange {
    x: (usize,usize),
    m: (usize,usize),
    a: (usize,usize),
    s: (usize,usize)
}

impl PartsRange {
    fn count(&self)-> usize {
        (self.x.1-self.x.0 +1)*
        (self.m.1-self.m.0 +1)*
        (self.a.1-self.a.0 +1)*
        (self.s.1-self.s.0 +1)
    }
}

fn get_accepted_combinations(ruleset: &HashMap<&str,(Vec<Rule<'_>>,Dest<'_>)>, partsrange: PartsRange, dest: Dest<'_>) -> usize {
    let (rules,dest) = match dest {
        Dest::Stop(c) => if c {return partsrange.count()} else {return 0},
        Dest::Next(name) => ruleset.get(name).unwrap()
    };
    let (mut count,partsrange) = rules.iter().fold((0,Some(partsrange)),|(mut count, partsrange), rule| {
        if let Some(partsrange) = partsrange {
            let (p1,partsrange) = rule.split_partsrange(partsrange);
            if let Some(partsrange) = p1 {
                count+= get_accepted_combinations(ruleset, partsrange, rule.dest)
            }
            return (count,partsrange)
        }
        (count,partsrange)
    });
    if let Some(partsrange) = partsrange {
        count+= get_accepted_combinations(ruleset, partsrange, *dest);
    }
    count
}

fn process_p2(path: impl AsRef<Path>) -> usize {
    let problem = std::fs::read_to_string(path).unwrap();
    let (ruleset,_) = problem.split_once("\n\n").unwrap();
    let ruleset = ruleset.lines().map(|line| {
        let (name, rem) = line.split_once('{').unwrap();
        let mut rules = rem.trim_end_matches('}')
            .split(',');
        let stop: Dest = rules.next_back().unwrap().into();
        let rules = rules.map(|s| {
                let (cond, dest) = s.split_once(':').unwrap();
                let value = cond[2..].parse::<usize>().unwrap();
                let part = s.chars().next().unwrap().into();
                let op = match s.chars().nth(1).unwrap() {
                    '>'=> Ordering::Greater,
                    '<'=> Ordering::Less,
                    _ => panic!("")
                };
                Rule {part,op,value,dest:dest.into()}
            }).collect::<Vec<_>>();
        (name,(rules,stop))
    }).collect::<HashMap<_,_>>();
    get_accepted_combinations(&ruleset, PartsRange { x: (1,4000), m: (1,4000), a: (1,4000), s: (1,4000) }, Dest::Next("in"))
}

#[test]
fn test_simpler() {
    let path = std::env::temp_dir().join("test_p2.dat");
    std::fs::write(&path, "in{a<2001:R,bug}
bug{a>3000:R,A}

").unwrap();
    assert_eq!(process_p2(path), 4000*4000*4000*1000)
}

#[test]
fn test_process_p2() {
    let path = std::env::temp_dir().join("test_p2.dat");
    std::fs::write(&path, "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}").unwrap();
    assert_eq!(process_p2(path), 167409079868000)
}

fn main() {
    let t0 = Instant::now();
    let result_p1 = process_p1("data/day19.txt");
    let t1 = Instant::now();
    println!("The result of p1 is {}. ({:?})", result_p1, t1 - t0);
    let result_p2 = process_p2("data/day19.txt");
    let t2 = Instant::now();
    println!("The result of p2 is {}. ({:?})", result_p2, t2 - t1);
}
