pub mod parse;
pub mod types; // Ensure types are also accessible

// Re-export the parse_gfa function for easier access
pub use parse::gfa::parse_gfa;
