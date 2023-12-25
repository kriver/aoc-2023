use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use rand::{distributions::Standard, prelude::*};

use crate::util::load;

// https://en.wikipedia.org/wiki/Cut_%28graph_theory%29
// https://en.wikipedia.org/wiki/Minimum_cut
// https://en.wikipedia.org/wiki/Stoer%E2%80%93Wagner_algorithm
// https://www.geeksforgeeks.org/introduction-and-implementation-of-kargers-algorithm-for-minimum-cut/

#[derive(Debug, Clone)]
struct Edge {
    src: usize,
    dst: usize,
}

#[derive(Debug)]
struct SubSet {
    parent: usize,
    rank: usize,
}

impl SubSet {
    fn new(parent: usize, rank: usize) -> Self {
        SubSet { parent, rank }
    }
}

#[derive(Debug)]
struct Graph {
    vertices: Vec<usize>,
    edges: Vec<Edge>,
}

impl Graph {
    fn new(vertices: Vec<usize>, edges: Vec<Edge>) -> Self {
        Graph { vertices, edges }
    }

    fn find(&mut self, subsets: &mut Vec<SubSet>, i: usize) -> usize {
        if subsets[i].parent != i {
            subsets[i].parent = self.find(subsets, subsets[i].parent);
        }
        subsets[i].parent
    }

    fn union(&mut self, subsets: &mut Vec<SubSet>, x: usize, y: usize) {
        let x = self.find(subsets, x);
        let y = self.find(subsets, y);

        if subsets[x].rank < subsets[y].rank {
            subsets[x].parent = y
        } else if subsets[x].rank > subsets[y].rank {
            subsets[y].parent = x;
        } else {
            subsets[y].parent = x;
            subsets[x].rank += 1;
        }
    }

    fn min_cut_karger(&mut self) -> (usize, HashMap<usize, HashSet<usize>>) {
        let mut num_v = self.vertices.len();
        let num_e = self.edges.len();
        let mut subsets = (0..num_v).map(|v| SubSet::new(v, 0)).collect::<Vec<_>>();
        while num_v > 2 {
            let mut i: usize = StdRng::from_entropy().sample(Standard);
            i %= num_e;

            let set1 = self.find(&mut subsets, self.edges[i].src);
            let set2 = self.find(&mut subsets, self.edges[i].dst);

            if set1 == set2 {
                continue;
            }

            num_v -= 1;
            self.union(&mut subsets, set1, set2);
        }
        let mut cuts = 0;
        let mut split: HashMap<usize, HashSet<usize>> = HashMap::new();
        for i in 0..num_e {
            let src = self.edges[i].src;
            let dst = self.edges[i].dst;
            let set1 = self.find(&mut subsets, src);
            let set2 = self.find(&mut subsets, dst);
            // println!("{},{} -> {} and {}", src, dst, set1, set2);
            if set1 != set2 {
                cuts += 1;
            } else {
                split
                    .entry(set1)
                    .and_modify(|c| {
                        c.insert(src);
                        c.insert(dst);
                    })
                    .or_insert(HashSet::from([src, dst]));
            }
        }
        (cuts, split)
    }
}

fn input(file: &str) -> Graph {
    let mut m = HashMap::new();
    let mut i: usize = 0;
    let mut vertices = HashSet::new();
    let mut edges = vec![];
    load::<String>(file).into_iter().for_each(|l| {
        let tokens = l
            .split([':', ' '])
            .filter(|t| !t.is_empty())
            .collect::<Vec<_>>();
        tokens.iter().for_each(|v| {
            if !m.contains_key(*v) {
                m.insert(v.to_string(), i);
                i += 1
            }
        });
        tokens.iter().for_each(|v| {
            vertices.insert(*m.get(*v).unwrap());
        });
        tokens[1..].into_iter().for_each(|v| {
            edges.push(Edge {
                src: *m.get(tokens[0]).unwrap(),
                dst: *m.get(*v).unwrap(),
            });
        });
    });
    Graph::new(Vec::from_iter(vertices.into_iter()), edges)
}

pub fn part1() -> usize {
    let mut g = input("data/day25.txt");
    let split = loop {
        let (cuts, split) = g.min_cut_karger();
        println!("Cuts {}", cuts);
        if cuts == 3 {
            break split;
        }
    };
    let split = split.into_values().collect_vec();
    println!("{} x {}", split[0].len(), split[1].len());
    split[0].len() * split[1].len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 589036);
    }
}
