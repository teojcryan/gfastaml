use std::io::Write;
use crate::types::{Node, Edge};

pub fn write_node<W: Write>(writer: &mut W, id: u32, node: &Node) -> Result<(), std::io::Error> {
    writeln!(writer, r#"        <node id="{}">"#, id)?;
    writeln!(writer, r#"            <data key="d0">{}</data>"#, node.seq)?;
    writeln!(writer, r#"      </node>"#)?;
    Ok(())
}

pub fn write_edge<W: Write>(writer: &mut W, edge: &Edge) -> Result<(), std::io::Error> {
    let orientation_code = match (edge.orientation.0, edge.orientation.1) {
        (true, true) => 0,
        (true, false) => 1,
        (false, true) => 2,
        (false, false) => 3,
    };
    writeln!(writer, r#"        <edge source="{}" target="{}">"#, edge.source, edge.target)?;
    writeln!(writer, r#"            <data key="d1">{}</data>"#, orientation_code)?;
    writeln!(writer, r#"      </edge>"#)?;
    Ok(())
}
