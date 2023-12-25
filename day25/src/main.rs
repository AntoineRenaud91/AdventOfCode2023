use nalgebra::{DMatrix, SymmetricEigen};
use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

#[cfg(test)]
const TEST_CASE: &str = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";

fn process_p1(data: &str) -> usize {
    let (_, graph, size) = data.lines().fold(
        (HashMap::new(), HashSet::new(), 0usize),
        |(mut map_i, mut graph, mut count), line| {
            let (source, targets) = line.split_once(':').unwrap();
            if !map_i.contains_key(source) {
                map_i.insert(source, count);
                count += 1;
            }
            let s = *map_i.get(source).unwrap();
            targets.split_whitespace().for_each(|target| {
                if !map_i.contains_key(target) {
                    map_i.insert(target, count);
                    count += 1
                }
                let t = *map_i.get(target).unwrap();
                graph.extend([(s, t), (t, s)]);
            });
            (map_i, graph, count)
        },
    );
    let mut lap = DMatrix::zeros(size, size);
    let diag = (0..size)
        .map(|i| {
            1. / (0..size)
                .map(|j| if graph.contains(&(i, j)) { 1. } else { 0. })
                .sum::<f64>()
                .sqrt()
        })
        .collect::<Vec<_>>();
    graph.iter().for_each(|&(i, j)| {
        lap[(i, j)] = -diag[i] * diag[j];
    });
    (0..size).for_each(|i| lap[(i, i)] += 1.);
    let eigen = SymmetricEigen::new(lap);
    let mut eigen_pairs: Vec<_> = eigen
        .eigenvalues
        .iter()
        .map(|e| e.abs())
        .zip(eigen.eigenvectors.column_iter())
        .collect();
    eigen_pairs
        .sort_by(|(eigenval_a, _), (eigenval_b, _)| eigenval_a.partial_cmp(eigenval_b).unwrap());
    let partition = eigen_pairs[1].1;
    partition.iter().filter(|v| v < &&0.).count() * partition.iter().filter(|v| v > &&0.).count()
}

#[test]
fn test_process_p1() {
    assert_eq!(process_p1(TEST_CASE), 54)
}

fn main() {
    let data = std::fs::read_to_string("data/day25.txt").unwrap();
    let t0 = Instant::now();
    let result_p1 = process_p1(&data);
    let t1 = Instant::now();
    println!("The result of p1 is {}. ({:?})", result_p1, t1 - t0);
}
