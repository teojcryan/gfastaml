use std::collections::HashMap;
use anyhow::{Context, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::types::{Node, Edge};

// Parses a GFA file and returns a HashMap of nodes and a vector of edges
pub fn parse_gfa(gfa_file: &str) -> Result<(HashMap<u32, Node>, Vec<Edge>)> {
    let mut nodes = HashMap::new(); // HashMap to store nodes by their IDs
    let mut edges = Vec::new(); // Vector to store edges

    println!("Opening GFA file: {}", gfa_file);
    let file = File::open(gfa_file).context("Failed to open GFA file")?;
    let reader = BufReader::new(file); // Buffer the file for efficient line-by-line reading

    // Iterate over each line in the GFA file
    for (line_number, line) in reader.lines().enumerate() {
        // Handle potential errors while reading a line
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                println!("Failed to read line {}: {}", line_number + 1, e);
                continue; // Skip the problematic line and continue parsing
            }
        };
        println!("Processing line {}: {}", line_number + 1, line);
        let parts: Vec<&str> = line.split_terminator('\t').collect(); // Split the line by tabs

        match parts[0] {
            "S" => { // Node definition line
                // Parse the node ID
                let node_name: u32 = match parts[1].parse() {
                    Ok(n) => n,
                    Err(e) => {
                        println!("Failed to parse node name as u32: {}", e);
                        continue; // Skip the problematic line if parsing fails
                    }
                };
                let seq = parts[2].to_string(); // Extract the sequence
                let seq_len = seq.len(); // Calculate the length of the sequence

                println!("Parsed node: id={}, seq_len={}", node_name, seq_len);
                nodes.insert(node_name, Node { id: node_name, seq, len: seq_len }); // Insert the node into the HashMap
            }
            "L" => { // Edge/link definition line
                // Parse the IDs of the nodes connected by the link
                let from_id: u32 = match parts[1].parse() {
                    Ok(id) => id,
                    Err(e) => {
                        println!("Failed to parse from node as u32: {}", e);
                        continue; // Skip the problematic line if parsing fails
                    }
                };
                let to_id: u32 = match parts[3].parse() {
                    Ok(id) => id,
                    Err(e) => {
                        println!("Failed to parse to node as u32: {}", e);
                        continue; // Skip the problematic line if parsing fails
                    }
                };
                let from_orient = parts[2]; // Orientation of the 'from' node
                let to_orient = parts[4]; // Orientation of the 'to' node

                // Validate the orientations to ensure they are either '+' or '-'
                if from_orient != "+" && from_orient != "-" {
                    println!("Invalid from orientation: {}", from_orient);
                    continue; // Skip if orientation is invalid
                }
                if to_orient != "+" && to_orient != "-" {
                    println!("Invalid to orientation: {}", to_orient);
                    continue; // Skip if orientation is invalid
                }

                println!("Parsed link: from_id={}, to_id={}, from_orient={}, to_orient={}", from_id, to_id, from_orient, to_orient);
                // Determine the orientation code for the edge based on the orientations of the nodes
                let orientation_code = match (from_orient, to_orient) {
                    ("+", "+") => 0,
                    ("+", "-") => 1,
                    ("-", "+") => 2,
                    ("-", "-") => 3,
                    _ => panic!("Invalid orientation"), // This should never happen due to earlier validation
                };

                edges.push(Edge { from: from_id, to: to_id, orientation: orientation_code }); // Add the edge to the vector
            }
            _ => {
                // Skip any unrecognized line types
                println!("Skipping unrecognized line type: {}", parts[0]);
                continue;
            }
        }
    }
    println!("Finished parsing GFA file. Parsed {} nodes and {} edges.", nodes.len(), edges.len());
    Ok((nodes, edges)) // Return the parsed nodes and edges
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Node, Edge};
    use std::collections::HashMap;
    use tempfile::NamedTempFile;
    use std::io::Write;

    #[test]
    fn test_parse_gfa() {
        // Define test GFA content
        let test_gfa_content = "\
        S\t1\tATG\n\
        S\t2\tCGT\n\
        L\t1\t+\t2\t-\n";

        // Write the test content to a temporary file
        let mut test_file = NamedTempFile::new().expect("Unable to create temp file");
        write!(test_file, "{}", test_gfa_content).expect("Unable to write test content");

        // Call the function with the test GFA file
        let (nodes, edges) = parse_gfa(test_file.path().to_str().unwrap()).expect("Failed to parse GFA file");

        // Check that the nodes are parsed correctly
        let mut expected_nodes = HashMap::new();
        expected_nodes.insert(1, Node { id: 1, seq: "ATG".to_string(), len: 3 });
        expected_nodes.insert(2, Node { id: 2, seq: "CGT".to_string(), len: 3 });

        assert_eq!(nodes, expected_nodes);

        // Check that the edges are parsed correctly
        let expected_edges = vec![
            Edge { from: 1, to: 2, orientation: 1 },
        ];

        assert_eq!(edges, expected_edges);
    }
}
