use std::collections::VecDeque;
use std::cmp::Ordering;
use std::fmt;
use std::rc::Rc;

pub mod river_sizes;
pub mod word_ladder;
pub mod prim_min_spanning_tree;

const EPSILON: f64 = 1e-10;

#[derive(Debug, Clone, Copy)]
pub struct Edge {
    v: usize,
    u: usize,
    weight: f64,
}

impl Edge {
    pub fn new(v: usize, u: usize, weight: f64) -> Self {
        Edge { v, u, weight }
    }

    pub fn either(&self) -> usize {
        self.v
    }

    pub fn other(&self, x: usize) -> usize {
        if x == self.v {
            return self.u;
        } else if x == self.u {
            return self.v;
        } 
        panic!("Illegal endpoint");
    }
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {} {:.2} ", self.v, self.u, self.weight)
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

#[derive(Debug, PartialEq)]
pub struct EdgeWeightedGraph {
    v: usize,
    e: usize,
    adj: Vec<VecDeque<Rc<Edge>>>,
}

impl EdgeWeightedGraph {
    pub fn new(v: usize) -> Self {
        let mut adj = vec![];
        for _ in 0..v {
            adj.push(VecDeque::new());
        }
        EdgeWeightedGraph {
            v,
            e: 0,
            adj
        }
    }

    pub fn add_edge(&mut self, edge: Edge) {
        let v = edge.either();
        let u = edge.other(v);
        let shared_edge = Rc::new(edge);
        self.adj[v].push_back(Rc::clone(&shared_edge));
        self.adj[u].push_back(Rc::clone(&shared_edge));
        self.e += 1;
    }

    pub fn adj(&self, v: usize) -> impl Iterator<Item = &Rc<Edge>> {
        self.adj[v].iter()
    }
    
}

impl fmt::Display for EdgeWeightedGraph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} {}", self.v, self.e)?;
        for v in 0..self.v {
            write!(f, "{}: ", v)?;
            for edge in &self.adj[v] {
                write!(f, "{} ", edge)?;
            }
            writeln!(f)?;
        }
        
        Ok(())
    }
}

fn nearly_equal(a: f64, b: f64, epsilon: f64) -> bool {
    let diff = (a - b).abs();
    let norm = f64::min(a.abs() + b.abs(), f64::MAX);
    diff / norm <= epsilon
}