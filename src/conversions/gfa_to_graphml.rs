// Conversion logic for GFA to GraphML

use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use crate::formats::graphml::{write_node, write_edge};
use crate::types::{Node, Edge};
use anyhow::Result;

pub fn convert_gfa_to_graphml(gfa_file: &str, output_file: &str) -> Result<()> {
    let file = File::open(gfa_file)?;
    let reader = BufReader::new(file);
    let output = File::create(output_file)?;
    let mut writer = BufWriter::new(output);

    // Write the GraphML header and keys
    writeln!(writer, r#"<?xml version="1.0" encoding="UTF-8"?>"#)?;
    writeln!(writer, r#"<graphml xmlns="http://graphml.graphdrawing.org/xmlns">"#)?;
    writeln!(writer, r#"    <key id="d0" for="node" attr.name="seq" attr.type="string"/>"#)?;
    writeln!(writer, r#"    <key id="d1" for="edge" attr.name="orientation" attr.type="int"/>"#)?;
    writeln!(writer, r#"    <graph id="G" edgedefault="directed">"#)?;

    // Process each line of the GFA file sequentially
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split('\t').collect();
        
        match parts[0] {
            // Process nodes (S lines)
            "S" => {
                if parts.len() < 3 {
                    continue; // Skip malformed lines
                }
                let id = parts[1].parse::<u32>()?;
                let seq = parts[2].to_string();
                let node = Node { id, seq };
                write_node(&mut writer, id, &node)?;
            }
            // Process edges (L lines)
            "L" => {
                if parts.len() < 5 {
                    continue; // Skip malformed lines
                }
                let source = parts[1].parse::<u32>()?;
                let target = parts[3].parse::<u32>()?;
                let source_orient = parts[2];
                let target_orient = parts[4];

                // Orientation tuple: true for positive (+), false for negative (-)
                let orientation = (
                    source_orient == "+",
                    target_orient == "+"
                );

                let edge = Edge { source, target, orientation };
                write_edge(&mut writer, &edge)?;
            }
            _ => {
                // Ignore other line types for now (e.g., headers or comments)
            }
        }
    }

    writeln!(writer, "  </graph>")?;
    writeln!(writer, "</graphml>")?;
    Ok(())
}
