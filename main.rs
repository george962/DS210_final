//import necessary libraries
mod graph;
use petgraph::dot::{Config, Dot};
use petgraph::Graph;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::process::Command;

//generates two files "graph.dot" and "graph.txt" which contain the DOT representation
//of the input graph
//one has edges and the other doesn't
pub fn generate_graph(graph: &Graph<i32, ()>) {
    let dot = format!("{:?}", Dot::with_config(graph, &[]));
    let mut file = File::create("graph.dot").expect("Unable to create file");
    write!(file, "{}", dot).expect("Unable to write file");

    let dot = Dot::with_config(graph, &[Config::EdgeNoLabel]);
    let dot_string = format!("{:?}", dot);
    let mut file = File::create("graph.txt").expect("Unable to create file");
    write!(file, "{}", dot_string).expect("Unable to write file");

}

//calculates the centrality of each node and writes the top 100 highest centrality to the "output.txt" file
pub fn calculate_centrality(graph: &Graph<i32,()>) {
    let num_nodes = graph.node_count() as f64;

    let mut node_centrality = HashMap::new();
    for node_index in graph.node_indices() {
        let degree = graph.neighbors(node_index).count() as f64;
        let centrality = degree / (num_nodes - 1.0);
        node_centrality.insert(node_index, centrality);
    }

    let mut sorted_centrality: Vec<_> = node_centrality.into_iter().collect();
    sorted_centrality.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    let mut file = File::create("output.txt").expect("Unable to create file");
    for (node_index, centrality) in sorted_centrality.iter().take(100) {
        writeln!(file, "{:?}: {}", node_index.index(), centrality).unwrap();
    }

}

//generates a graph image from "graph.txt" using the Graphviz 'dot' command
//image saved as "graph.png"
pub fn generate_image() {
    let output = Command::new("/opt/homebrew/bin/dot")
        .arg("-Tpng")
        .arg("-Gdpi=300")
        .arg("graph.txt")
        .arg("-o")
        .arg("graph.png")
        .output()
        .expect("failed to execute process");
    
    if output.status.success() {
        println!("Generated graph image: graph.png");
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        println!("Failed to generate graph image: {}", error);
    }
}

//uses the functions to complete the program
pub fn main() {
    let graph = graph::create_graph();

    let mut dot = String::new();
    dot.push_str("digraph {\n");

    for node_index in graph.node_indices() {
        let node_label = graph.node_weight(node_index).unwrap();
        dot.push_str(&format!("{} [ label = {} ]\n", node_index.index(), node_label));
    }
    
    for edge in graph.edge_indices() {
        let (source, target) = graph.edge_endpoints(edge).unwrap();
        dot.push_str(&format!("{} -> {}\n", source.index(), target.index()));
    }

    dot.push_str("}\n");

    generate_graph(&graph);
    calculate_centrality(&graph);
    generate_image();
    println!("t")

}

//tests to verify that the function "generate_image" and "generate_graph" have the expected output
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_generate_image() {
        generate_image();
        assert!(std::path::Path::new("src/graph.png").exists());
    }

    #[test]
    fn test_generate_graph() {
        let graph = graph::create_graph();
        generate_graph(&graph);
        assert!(std::path::Path::new("graph.dot").exists());
        assert!(std::path::Path::new("graph.txt").exists());
    }
}
