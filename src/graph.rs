use std::collections::VecDeque;
use std::cmp::Ordering;
use std::fmt;

pub mod river_sizes;
pub mod word_ladder;

const EPSILON: f64 = 1e-10;

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

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {} {:.2}", self.v, self.u, self.weight)
    }
}

impl Eq for Edge {}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.v.eq(&other.v) && self.u.eq(&other.u) && nearly_equal(self.weight, other.weight, EPSILON)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.weight < other.weight {
            Ordering::Less
        } else if self.weight > other.weight {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

#[derive(Debug)]
pub struct EdgeWeightedGraph {
    v: i32,
    e: i32,
    adj: Vec<VecDeque<Edge>>,
}

impl EdgeWeightedGraph {
    pub fn new() -> Self {
        todo!()
    }

    pub fn from_file(file_path: &str) -> Self {
        todo!()
    }
}

fn nearly_equal(a: f64, b: f64, epsilon: f64) -> bool {
    let diff = (a - b).abs();
    let norm = f64::min(a.abs() + b.abs(), f64::MAX);
    diff / norm <= epsilon
}