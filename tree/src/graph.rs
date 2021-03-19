use std::cmp::Ordering;
use std::collections::HashSet;
use std::hash::Hash;
use std::ops::Add;

use crate::graph::Distance::{Infinite, Number};

pub struct Graph<K: Eq + Hash> {
    adjacency_list: Vec<Vec<Edge>>,
    nodes: Vec<K>,
}

#[derive(Clone)]
struct Edge {
    weight: u32,
    node: usize,
}

impl<K: Eq + Hash> Graph<K> {
    pub fn get_node_index(&self, key: &K) -> Option<usize> {
        self.nodes.iter().position(|node| node == key)
    }
    pub fn set_edges(&mut self, from: K, edges: Vec<(u32, K)>) {
        let edges: Vec<Edge> = edges
            .iter()
            .filter_map(|(weight, key)| {
                self.get_node_index(key).map(|index| Edge {
                    weight: *weight,
                    node: index,
                })
            })
            .collect();
        match self.get_node_index(&from) {
            None => {
                self.nodes.push(from);
                self.adjacency_list.push(edges);
            }
            Some(from) => {
                self.adjacency_list[from] = edges;
            }
        }
    }
    pub fn set_nodes(&mut self, nodes: Vec<K>) {
        self.adjacency_list = vec![vec![]; nodes.len()];
        self.nodes = nodes;
    }
    pub fn connected(&self, from: &K, degree: usize) -> Option<HashSet<&K>> {
        self.nodes.iter().position(|n| n == from).map(|from| {
            self.connected_r(from, degree)
                .iter()
                .map(|&index| &self.nodes[index])
                .collect()
        })
    }
    fn connected_r(&self, from: usize, degree: usize) -> HashSet<usize> {
        if degree == 0 {
            return HashSet::new();
        }
        self.adjacency_list[from]
            .iter()
            .flat_map(|edge| {
                let mut r = self.connected_r(edge.node, degree - 1);
                r.insert(edge.node);
                r
            })
            .collect()
    }

    pub fn shortest_path(&self, from: &K, to: &K) -> Option<(u32, Vec<&K>)> {
        let (from, to) =
            if let (Some(from), Some(to)) = (self.get_node_index(from), self.get_node_index(to)) {
                (from, to)
            } else {
                return None;
            };
        let mut distances = vec![Distance::Infinite; self.nodes.len()];
        distances[from] = Distance::Number(0);
        let mut open: Vec<usize> = (0..self.nodes.len()).collect();
        let mut parent = vec![None; self.nodes.len()];
        let mut found = false;
        while !open.is_empty() {
            let (min_index_in_open, min_distance) = Self::min_index_in_open(&distances, &open);
            let min_index_in_nodes = open.swap_remove(min_index_in_open);
            if min_index_in_nodes == to {
                found = true;
                break;
            }

            for edge in &self.adjacency_list[min_index_in_nodes] {
                let new_distance = min_distance + edge.weight;
                if new_distance < distances[edge.node] {
                    distances[edge.node] = new_distance;
                    parent[edge.node] = Some(min_index_in_nodes)
                }
            }
        }

        if found {
            None
        } else {
            let mut cur = to;
            let mut path = vec![&self.nodes[cur]];
            while let Some(parent) = parent[cur] {
                path.push(&self.nodes[parent]);
                cur = parent;
            }
            path.reverse();
            let distance = match distances[to] {
                Infinite => {
                    unreachable!()
                }
                Number(distance) => distance,
            };
            Some((distance, path))
        }
    }
    fn min_index_in_open(distances: &[Distance], open: &[usize]) -> (usize, Distance) {
        assert!(!open.is_empty());
        let (mut min_distance, mut min_index) = (distances[open[0]], open[0]);
        for index in open.iter() {
            let distance = distances[*index];
            if distance < min_distance {
                min_distance = distance;
                min_index = *index;
            }
        }
        (min_index, min_distance)
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Distance {
    Infinite,
    Number(u32),
}

impl PartialOrd for Distance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Distance::Infinite, Distance::Infinite) => Some(Ordering::Equal),

            (Distance::Number(_weight), Distance::Infinite) => Some(Ordering::Less),
            (Distance::Infinite, Distance::Number(_weight)) => Some(Ordering::Greater),
            (Distance::Number(weight_self), Distance::Number(weight_other)) => {
                weight_self.partial_cmp(weight_other)
            }
        }
    }
}

impl Ord for Distance {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Add for Distance {
    type Output = Distance;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number(self_number), Number(rhs_number)) => Number(self_number.add(rhs_number)),
            _ => Infinite,
        }
    }
}

impl Add<u32> for Distance {
    type Output = Distance;

    fn add(self, rhs: u32) -> Self::Output {
        match self {
            Number(self_number) => Number(self_number.add(rhs)),
            _ => Infinite,
        }
    }
}

impl Add<Distance> for u32 {
    type Output = Distance;

    fn add(self, rhs: Distance) -> Self::Output {
        match rhs {
            Number(rhs_number) => Number(self.add(rhs_number)),
            _ => Infinite,
        }
    }
}
