use std::collections::VecDeque;
use std::cmp::Ordering;
use std::fmt;
use std::path::PathBuf;
use std::rc::Rc;
use std::fs::File;
use std::io::{self, BufRead, Error, ErrorKind};

pub mod river_sizes;
pub mod word_ladder;
pub mod prim_min_spanning_tree;
pub mod breadth_first_search;
pub mod depth_first_search;
mod all_paths_from_source_target;
mod a_star_algorithm;

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

    pub fn weight(&self) -> f64 {
        self.weight
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

#[derive(Debug)]
pub struct DirectedEdge {
    v: usize,
    u: usize,
    weight: f64
}

impl DirectedEdge {
    pub fn new(v: usize, u: usize, weight: f64) -> Self {
        DirectedEdge {
            v,
            u,
            weight,
        }
    }

    pub fn from(&self) -> usize {
        self.v
    }

    pub fn to(&self) -> usize {
        self.u
    }

    pub fn weight(&self) -> f64 {
        self.weight
    }

}

impl Eq for DirectedEdge {}

impl PartialEq for DirectedEdge {
    fn eq(&self, other: &Self) -> bool {
        self.v.eq(&other.v) && self.u.eq(&other.u) && nearly_equal(self.weight, other.weight, EPSILON)
    }
}

impl fmt::Display for DirectedEdge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {} {:.2} ", self.v, self.u, self.weight)
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

#[derive(Debug, PartialEq)]
pub struct EdgeWeightedDigraph {
    v: usize,
    e: usize,
    adj: Vec<VecDeque<Rc<DirectedEdge>>>,
}

impl EdgeWeightedDigraph {
    pub fn new(v: usize) -> Self {
        let adj = vec![VecDeque::new(); v];
        EdgeWeightedDigraph {
            v,
            e: 0,
            adj
        }
    }

    pub fn from_file(path: PathBuf) -> Result<Self, std::io::Error> {
        
        let file = File::open(path)?;
        let reader = io::BufReader::new(file);

        let mut iterator = reader.lines();
        let v: usize = iterator.next()
            .ok_or(Error::new(ErrorKind::InvalidData, "number of vertices not found in the file"))
            .and_then(|x| x)
            .and_then(|x| x.parse().map_err(|e| Error::new(ErrorKind::InvalidInput, e)))?;

        let _ = iterator.next();
            // .ok_or(Error::new(ErrorKind::InvalidData, "number of edges not found in the file"))
            // .and_then(|x| x)
            // .and_then(|x| x.parse().map_err(|e| Error::new(ErrorKind::InvalidInput, e)))?;

        let mut graph = EdgeWeightedDigraph::new(v);

        for line in iterator {
            let line: String = line?;
            let edge_parts: Vec<&str> = line.split_whitespace().collect();
            let from: usize = edge_parts[0].parse().map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;
            let to: usize = edge_parts[1].parse().map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;
            let weight: f64 = edge_parts[2].parse().map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;
            graph.add_edge(DirectedEdge::new(from, to, weight));
        }
        

        Ok(graph)
    }

    pub fn add_edge(&mut self, edge: DirectedEdge) {
        let v = edge.from();
        let e = Rc::new(edge);
        self.adj[v].push_back(Rc::clone(&e));
        self.e += 1;
    }

    pub fn edges(&self) -> Vec<Rc<DirectedEdge>> {
        let mut res = vec![];
        for v in 0..self.v {
            for edge in &self.adj[v] {
                res.push(Rc::clone(edge));
            }
        }

        res
    }

    pub fn adj(&self, v: usize) -> impl Iterator<Item = &Rc<DirectedEdge>> {
        self.adj[v].iter()
    }
}

impl fmt::Display for EdgeWeightedDigraph {
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