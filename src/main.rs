use clap::Parser;
use anyhow::Result;
use gfastaml::conversions::{gfa_to_graphml::convert_gfa_to_graphml, fasta_to_gfa::convert_fasta_to_gfa, fasta_to_graphml::convert_fasta_to_graphml};

/// Command-line arguments struct using clap
#[derive(Parser)]
#[command(name = "gfastaml", about = "A tool for parsing and converting files")]
struct Args {
    /// Input file
    #[arg(short, long)]
    input_file: String,

    /// Output file
    #[arg(short, long)]
    output_file: String,

    /// Conversion type
    #[arg(short, long)]
    conversion: String, // Options: "gfa_to_graphml", "fasta_to_gfa", "fasta_to_graphml"
}

fn main() -> Result<()> {
    // Parse command-line arguments
    let args = Args::parse();

    match args.conversion.as_str() {
        "gfa_to_graphml" => convert_gfa_to_graphml(&args.input_file, &args.output_file)?,
        "fasta_to_gfa" => convert_fasta_to_gfa(&args.input_file, &args.output_file)?,
        "fasta_to_graphml" => convert_fasta_to_graphml(&args.input_file, &args.output_file)?,
        _ => {
            eprintln!("Invalid conversion type. Use 'gfa_to_graphml', 'fasta_to_gfa', or 'fasta_to_graphml'.");
            std::process::exit(1);
        }
    }

    println!("Conversion completed. Output written to {}", args.output_file);
    Ok(())
}
