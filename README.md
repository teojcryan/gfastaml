# gfastaml
A Rust package for converting between GFA, FASTA, and GraphML formats. Easily transform sequence and graph representations for genomic data analysis.

## Features
- Convert between graphical fragment assembly (.gfa), .fasta, and .graphml formats

## Usage
`gfastaml` is a command-line tool that allows conversion between `.gfa`, `.fasta`, and `.graphml` formats. 

```sh
# Convert a GFA file to GraphML
gfastaml convert --input mygraph.gfa --output mygraph.graphml

# Convert a FASTA file to GFA
gfastaml convert --input myunitigs.fasta --output mygraph.gfa

# Convert a BCALM FASTA file to GraphML
gfastaml convert --input myunitigs.fasta --output mygraph.graphml
```

### Command-Line Arguments
- `-i, --input`: The input file (in `.gfa`, `.fasta`, or `.graphml` format).
- `-o, --output`: The desired output file with the format specified by its extension.
