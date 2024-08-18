use std::collections::VecDeque;

pub mod river_sizes;
pub mod word_ladder;

#[derive(Debug)]
pub struct Edge {
    v: i32,
    u: i32,
    weight: f64,
}

impl Edge {
    pub fn new(v: i32, u: i32, weight: f64) -> Self {
        Edge { v, u, weight }
    }
}

#[derive(Debug)]
pub struct EdgeWeightedGraph {
    v: i32,
    e: i32,
    adj: Vec<VecDeque<Edge>>,
}