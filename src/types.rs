#[derive(Debug, PartialEq, Eq)]
pub struct Node {
    pub id: u32,
    pub seq: String,
    pub len: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Edge {
    pub from: u32,
    pub to: u32,
    pub orientation: u8,
}
