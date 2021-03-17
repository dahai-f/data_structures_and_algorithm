use std::collections::HashSet;
use std::hash::Hash;

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
}
