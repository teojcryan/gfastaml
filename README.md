# gfastaml
A Rust package for converting between GFA, FASTA, and GraphML formats. Easily transform sequence and graph representations for genomic data analysis.

## Features
- Convert between graphical fragment assembly (.gfa), .fasta, and .graphml formats

Specifically,
- GFA files as specified [here](https://github.com/GFA-spec/GFA-spec/blob/master/GFA1.md) that contain only
	- #: Comment lines
	- H: Header lines
	- S: Segment lines denoting the nucleotide sequence
	- L: Link lines denoting the bidirected edges linking segments
- .fasta files as specified in [bcalm2 format](https://github.com/GATB/bcalm/blob/master/bidirected-graphs-in-bcalm2/bidirected-graphs-in-bcalm2.md#bcalm-2)
	- These are regular .fasta files except graph edges (or links in .gfa) are encoded by `L:<e.fromSign>:<e.to>:<e.toSign>` in the header
- Graph Markup Language (.graphml) files are defined as [such](http://graphml.graphdrawing.org/specification/xsd.html)
	- This is explained more simply [here](https://gephi.org/users/supported-graph-formats/graphml-format/). 

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
