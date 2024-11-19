use std::collections::HashMap;
use std::fs::{File};
use std::io::{Write, BufWriter};
use crate::types::{Node, Edge};

pub fn write_graphml(nodes: HashMap<u32, Node>, edges: Vec<Edge>, output_file: &str) -> Result<(), std::io::Error> {
    let file = File::create(output_file)?;
    let mut writer = BufWriter::new(file);

    // Write the GraphML header and keys
    writeln!(writer, r#"<?xml version="1.0" encoding="UTF-8"?>"#)?;
    writeln!(writer, r#"<graphml xmlns="http://graphml.graphdrawing.org/xmlns">"#)?;
    writeln!(writer, r#"<key id="d0" for="node" attr.name="seq" attr.type="string"/>"#)?;
    writeln!(writer, r#"<key id="d1" for="edge" attr.name="orientation" attr.type="int"/>"#)?;
    writeln!(writer, r#"<graph id="G" edgedefault="directed">"#)?;

    // Write nodes to the GraphML file
    for (id, node) in nodes {
        write_node(&mut writer, id, &node)?;
    }

    // Write edges to the GraphML file
    for edge in edges {
        write_edge(&mut writer, &edge)?;
    }

    // Write the closing tags for GraphML
    writeln!(writer, "</graph>")?;
    writeln!(writer, "</graphml>")?;
    println!("GraphML file written to {}", output_file);
    Ok(())
}

fn write_node<W: Write>(writer: &mut W, id: u32, node: &Node) -> Result<(), std::io::Error> {
    writeln!(writer, r#"<node id="{}">"#, id)?;
    writeln!(writer, r#"<data key="d0">{}</data>"#, node.seq)?;
    writeln!(writer, "</node>")?;
    Ok(())
}

fn write_edge<W: Write>(writer: &mut W, edge: &Edge) -> Result<(), std::io::Error> {
    writeln!(writer, r#"<edge source="{}" target="{}">"#, edge.from, edge.to)?;
    writeln!(writer, r#"<data key="d1">{}</data>"#, edge.orientation)?;
    writeln!(writer, "</edge>")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Node, Edge};
    use std::collections::HashMap;
    use tempfile::NamedTempFile;

    #[test]
    fn test_write_graphml() {
        // Create test nodes and edges
        let mut nodes = HashMap::new();
        nodes.insert(1, Node { id: 1, seq: "ATG".to_string() });
        nodes.insert(2, Node { id: 2, seq: "CGT".to_string() });

        let edges = vec![
            Edge { from: 1, to: 2, orientation: 1 },
        ];

        // Create a temporary file to write the GraphML output
        let output_file = NamedTempFile::new().expect("Unable to create temp file");
        let output_path = output_file.path().to_str().unwrap().to_string();

        // Call the write_graphml function
        write_graphml(nodes, edges, &output_path).expect("Failed to write GraphML file");

        // Read the written file and verify its content
        let written_content = read_to_string(output_path).expect("Unable to read the GraphML file");

        assert!(written_content.contains(r#"<node id="1">"#));
        assert!(written_content.contains(r#"<data key="d0">ATG</data>"#));
        assert!(written_content.contains(r#"<node id="2">"#));
        assert!(written_content.contains(r#"<data key="d0">CGT</data>"#));
        assert!(written_content.contains(r#"<edge source="1" target="2">"#));
        assert!(written_content.contains(r#"<data key="d1">1</data>"#));
    }
}
