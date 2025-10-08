use petgraph::{adj::NodeIndex, Graph, Directed, graphmap::DiGraphMap, graphmap::GraphMap};
use petgraph::graph::IndexType;
use std::collections::HashMap;
use std::hash::Hash;

pub struct GraphAnalysis {
    pub graph: Graph<String, String>,
    pub node_map: HashMap<String, NodeIndex>,
}

impl GraphAnalysis {
    pub fn new(graph: Graph<String, String>) -> GraphAnalysis {
        let g = GraphAnalysis {
            graph: graph,
            node_map: HashMap::<String, NodeIndex>::new(),
        };

        return g;
    }

    pub fn setup_node_map(&mut self) -> () {
        let mut node_map: HashMap<String, NodeIndex> = HashMap::<String, NodeIndex>::new();
        let mut index: usize = 0;
        for node in self.graph.node_weights() {
            let node_index: NodeIndex = NodeIndex::new(index);
            //let node_name = self.graph[node_index].clone();
            node_map.insert(node.to_string(), node_index);
            index += 1;
        }

        self.node_map = node_map;
    }

    pub fn print_node_map(&mut self) -> () {
        for (key, value) in self.node_map.iter() {
            println!("Node: {}, Index: {}", key, value.index());
        }
    }

    pub fn analyze(&self) {
        // Placeholder for graph analysis logic
        println!("\nNode Count: {}\nEdge Count: {}\n", self.graph.node_count(), self.graph.edge_count());
        self.graph.raw_nodes().iter().enumerate().for_each(|(i, node)| {
            println!("Node {}: {:?}", i, node);
        });
    }


    /* 
        User functions that are helpful for manual analysis 
    */

    pub fn user_get_neighbors(&self, node: &str) -> Vec<String> {
        let mut neighbors = Vec::<String>::new();
        let index: NodeIndex = match self.node_map.get(node) {
            Some(&idx) => idx,
            None => {
                eprintln!("Node '{}' not found in the graph.", node);
                return neighbors;
            }
        };
        for node in self.graph.neighbors_undirected(index.into()) {
            neighbors.push(self.graph[node].clone());
        }

        return neighbors;

    }

    pub fn user_logic_cone(&self, node: &str, levels: u8, direction: &str) -> Vec<String> {
        let mut direct = petgraph::Direction::Outgoing;
        if direction == "in" {
            direct = petgraph::Direction::Incoming;
        }

        let mut cone = Vec::<String>::new();

        let index: NodeIndex = match self.node_map.get(node) {
            Some(&idx) => idx,
            None => {
                eprintln!("Node '{}' not found in the graph.", node);
                return cone;
            }
        };
        for neighbor in self.graph.neighbors_directed(index.into(), direct) {
            cone.push(self.graph[neighbor].clone());
        }

        let mut current_level = cone.clone();

        for level in 1..levels {
            let mut next_level = Vec::<String>::new();
            for current_node in current_level.iter() {
                println!("Current Level Node: {}", current_node);
                let idx: NodeIndex = match self.node_map.get(current_node) {
                    Some(&idx) => idx,
                    None => {
                        eprintln!("Node '{}' not found in the graph.", current_node);
                        println!("Skipping to next node.");
                        continue;
                    }
                };
                println!("\nCurrent Index: {}", idx);
                println!("Neighbors count: {}", self.graph.neighbors_directed(idx.into(), direct).count());
                for n in self.graph.neighbors_directed(idx.into(), direct) {
                    println!("blah");
                    let neighbor_name = self.graph[n].clone();
                    println!("Neighbor: {}", neighbor_name);
                    next_level.push(neighbor_name);
                }
                println!("after for loop");
            }
            println!("\nLevel {}: {:?}", level, next_level);
            current_level = next_level;
            cone.append(&mut current_level);
        }

        return cone;
    }

}