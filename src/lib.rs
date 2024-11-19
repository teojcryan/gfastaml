pub mod parse;
pub mod write;
pub mod types; 

// Re-export functions for easier access
pub use parse::gfa::parse_gfa;
pub use write::graphml::write_graphml;
