use clap::Parser;
use anyhow::Result;
use gfastaml::parse::gfa::parse_gfa;
use gfastaml::write::graphml::write_graphml;

/// Command-line arguments struct using clap
#[derive(Parser)]
#[command(name = "gfastaml", about = "A tool for parsing GFA files and converting them")]
struct Args {
    /// Input GFA file
    #[arg(short, long)]
    gfa_file: String,

    /// Output GraphML file
    #[arg(short, long)]
    output_file: String,
}

fn main() -> Result<()> {
    // Parse command-line arguments
    let args = Args::parse();

    // Call the parse function with the specified GFA file
    let (nodes, edges) = parse_gfa(&args.gfa_file)?;

    println!("Parsed {} nodes and {} edges.", nodes.len(), edges.len());

    // Call the write_graphml function with the parsed nodes and edges
    write_graphml(nodes, edges, &args.output_file)?;
    
    println!("GraphML file written to {}", args.output_file);
    Ok(())
}
