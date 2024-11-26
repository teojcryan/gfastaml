use std::fs::File;
use std::io::{BufRead, BufReader, Write, BufWriter, Result};
use crate::types::{Node, Edge};
use crate::formats::gfa::write::{write_node, write_edge};

pub fn parse_gfa(gfa_file: &str, output_file: &str) -> Result<()> {
    // Open the GFA file for reading
    let file = File::open(gfa_file)?;
    let reader = BufReader::new(file);

    // Open the output GraphML file for writing
    let output = File::create(output_file)?;
    let mut writer = BufWriter::new(output);

    // Write the GraphML header and keys
    writeln!(writer, r#"<?xml version="1.0" encoding="UTF-8"?>"#)?;
    writeln!(writer, r#"<graphml xmlns="http://graphml.graphdrawing.org/xmlns">"#)?;
    writeln!(writer, r#"<key id="d0" for="node" attr.name="seq" attr.type="string"/>"#)?;
    writeln!(writer, r#"<key id="d1" for="edge" attr.name="orientation" attr.type="int"/>"#)?;
    writeln!(writer, r#"<graph id="G" edgedefault="directed">"#)?;

    // Process each line of the GFA file
    for line in reader.lines() {
        let line = line?;

        // Split the line and determine its type
        let parts: Vec<&str> = line.split('\t').collect();
        match parts[0] {
            "S" => {
                let id = parts[1].parse::<u32>()?;
                let seq = parts[2].to_string();
                write_node(&mut writer, id, &Node { id, seq })?;
            }
            "L" => {
                let from = parts[1].parse::<u32>()?;
                let to = parts[3].parse::<u32>()?;
                let orientation = match parts[2] {
                    "+" => 1,
                    "-" => -1,
                    _ => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid orientation")),
                };
                write_edge(&mut writer, &Edge { from, to, orientation })?;
            }
            _ => {}
        }
    }

    // Write the closing tags for GraphML
    writeln!(writer, "</graph>")?;
    writeln!(writer, "</graphml>")?;
    Ok(())
}
