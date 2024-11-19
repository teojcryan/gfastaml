use clap::Parser;
use anyhow::Result;
use gfastaml::parse::gfa::parse_gfa;

/// Command-line arguments struct using clap
#[derive(Parser)]
#[command(name = "gfastaml", about = "A tool for parsing GFA files and converting them")]
struct Args {
    /// Input GFA file
    #[arg(short, long)]
    gfa_file: String,
}

fn main() -> Result<()> {
    // Parse command-line arguments
    let args = Args::parse();

    // Call the parse function with the specified GFA file
    let (nodes, edges) = parse_gfa(&args.gfa_file)?;

    println!("Parsed {} nodes and {} edges.", nodes.len(), edges.len());
    Ok(())
}
