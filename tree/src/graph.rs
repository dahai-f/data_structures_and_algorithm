pub struct Graph<K: Eq> {
    adjacency_list: Vec<Vec<Edge>>,
    nodes: Vec<K>,
}

struct Edge {
    weight: u32,
    node: usize,
}

impl<K: Eq> Graph<K> {
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
}
