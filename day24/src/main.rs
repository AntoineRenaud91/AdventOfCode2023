use std::{time::Instant, collections:: HashMap};
use nalgebra::{Vector2,Matrix2, vector, Vector3, Matrix3, Matrix6, Vector6};

#[cfg(test)]
const TEST_DATA: &str ="19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";


#[derive(Debug)]
struct HailStone2 {
    pos: Vector2<f64>,
    vel: Vector2<f64>
}

impl HailStone2 {
    fn intersect_path(&self, other: &Self) -> Option<Vector2<f64>> {
        let m = Matrix2::from_columns(&[self.vel,-other.vel]);
        let rhs = other.pos-self.pos;
        m.try_inverse().map(|m| (m*rhs)[0]*self.vel+self.pos)
    }

    fn intersect_path_in_domain(&self, other: &Self, domain: &[f64;2]) -> bool {
        if let Some(p) = self.intersect_path(other) {
            (p[0]-self.pos[0])*self.vel[0]>=0. &&
            (p[0]-other.pos[0])*other.vel[0]>=0. &&  
            p[0]>= domain[0] && p[0] <= domain[1] &&
            p[1]>= domain[0] && p[1] <= domain[1]
        } else {
            false
        }

    }
}

#[test]
fn test_intersect_path(){
    let p1 = HailStone2 {pos: vector![0.,1.],vel: vector![1.,-1.]};
    let p2 = HailStone2 {pos: vector![0.,0.],vel: vector![1.,1.]};
    assert_eq!(p1.intersect_path(&p2),Some(vector![0.5,0.5]))
}

fn process_p1(data: &str,domain:&[f64;2]) -> usize {
    let hailstones = data.lines()
        .map(|line| {
            let (pos,vel) = line.split_once('@').unwrap();
            let mut pos = pos.split(',').map(|n| n.trim().parse().unwrap());
            let mut vel = vel.split(',').map(|n| n.trim().parse().unwrap());
            HailStone2 {
                pos: vector![
                    pos.next().unwrap(),
                    pos.next().unwrap()
                ],
                vel: vector![
                    vel.next().unwrap(),
                    vel.next().unwrap()
                ]
            }
        }).collect::<Vec<_>>();
    (0..hailstones.len()-1)
        .flat_map(|i| (i+1..hailstones.len()).map(move |j| (i,j)))
        .filter(|&(i,j)| hailstones[i].intersect_path_in_domain(&hailstones[j], domain))
        .count()
}

#[test]
fn test_process_p1() {
    assert_eq!(process_p1(TEST_DATA,&[7.,27.]), 2)
}


#[derive(Debug,Clone,Copy)]
struct HailStone3 {
    pos: Vector3<f64>,
    vel: Vector3<f64>
}

impl HailStone3 {
    fn delta_cross_op(&self, other: &Self) -> [Matrix3<f64>;2] {
        [(self.vel-other.vel).cross_matrix(),
         (other.pos-self.pos).cross_matrix()]
    }
    fn delta_cross_vec(&self, other: &Self) -> Vector3<f64> {
        other.pos.cross(&other.vel)-self.pos.cross(&self.vel)
    }
}

fn get_linear_problem([a,b,c]:[HailStone3;3]) -> (Matrix6<f64>,Vector6<f64>) {
    let mut m = Matrix6::zeros();
    let mut y = Vector6::zeros();
    let [m00,m01] = a.delta_cross_op(&b);
    let [m10,m11] = a.delta_cross_op(&c);
    let y0 = a.delta_cross_vec(&b);
    let y1 = a.delta_cross_vec(&c);
    (0..3).for_each(|i| {
        y[i]=y0[i];
        y[i+3]=y1[i];
        (0..3).for_each(|j| {
            m[(i,j)] = m00[(i,j)];
            m[(i,j+3)] = m01[(i,j)];
            m[(i+3,j)] = m10[(i,j)];
            m[(i+3,j+3)] = m11[(i,j)];
        })});
    (m,y)
}

fn process_p2(data: &str) -> i64 {
    let hailstones = data.lines()
        .map(|line| {
            let (pos,vel) = line.split_once('@').unwrap();
            let mut pos = pos.split(',').map(|n| n.trim().parse().unwrap());
            let mut vel = vel.split(',').map(|n| n.trim().parse().unwrap());
            HailStone3 {
                pos: vector![
                    pos.next().unwrap(),
                    pos.next().unwrap(),
                    pos.next().unwrap()
                ],
                vel: vector![
                    vel.next().unwrap(),
                    vel.next().unwrap(),
                    vel.next().unwrap()
                ]
            }
        }).collect::<Vec<_>>();
    let nh = hailstones.len();
    let counts=(0..nh-2).flat_map(|i| 
        ((i+1)..nh-1).flat_map(move |j| 
            ((j+1)..nh).map(move |k| (i,j,k))
        )
    ).filter_map(|(i,j,k)| {
        let (m,y) = get_linear_problem([hailstones[i],hailstones[j],hailstones[k]]);
        let r = m.try_inverse()?*y;
        Some((r[0]+r[1]+r[2]).round() as i64)
    }).fold(HashMap::new(),|mut map,c| {
        if let Some(n) = map.get_mut(&c) {
            *n+=1;
        } else {
            map.insert(c, 1usize);
        }
        map
    });
    let (c,n) = counts.iter().max_by_key(|(_,n)| *n).unwrap();
    // Some issue in my inputs, perhaps some floating point number errors.. 
    // Decided to count to get the highest occurance.
    println!("count {c} occured: {}%",(*n as f64)/(counts.values().sum::<usize>() as f64)*100.);
    *c
}

#[test]
fn test_process_p2() {
    assert_eq!(process_p2(TEST_DATA), 47)
}

fn main() {
    let t0 = Instant::now();
    let result_p1 = process_p1(&std::fs::read_to_string("data/day24.txt").unwrap(),&[200000000000000.,400000000000000.]);
    let t1 = Instant::now();
    println!("The result of p1 is {}. ({:?})", result_p1, t1 - t0);
    let result_p2 = process_p2(&std::fs::read_to_string("data/day24.txt").unwrap());
    let t2 = Instant::now();
    println!("The result of p2 is {}. ({:?})", result_p2, t2 - t1);
}
