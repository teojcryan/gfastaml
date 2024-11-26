#[derive(Debug, PartialEq, Eq)]
pub struct Node {
    pub id: u32,
    pub seq: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Edge {
    pub source: u32,
    pub target: u32,
    pub orientation: (bool, bool),
}
