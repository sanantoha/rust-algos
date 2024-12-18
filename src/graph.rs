use std::hash::{Hash, Hasher};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::cmp::Ordering;
use std::fmt;
use std::fmt::Display;
use std::path::PathBuf;
use std::rc::Rc;
use std::fs::File;
use std::io::{self, BufRead, Error, ErrorKind};
use itertools::Itertools;

pub mod river_sizes;
pub mod word_ladder;
pub mod prim_min_spanning_tree;
pub mod breadth_first_search;
pub mod depth_first_search;
mod all_paths_from_source_target;
mod a_star_algorithm;
mod bellman_ford;
mod clone_graph;
mod breadth_first_search_as_map;
mod depth_first_search_as_map;
mod prim_min_spanning_tree_as_map;
mod dijkstra_shortest_paths;
mod kruskal_min_spanning_tree;
mod largest_island;
mod longest_increasing_path_in_matrix;
mod munimum_passes_of_matrix;
mod number_of_islands;
mod surround_regions;
mod topological_sort;
mod topological_sort_dfs_cycle_graph;
mod word_ladder_ii;
mod bellman_ford_as_map;
mod dijkstra_shortest_paths_as_map;
mod kruskal_min_spanning_tree_as_map;
mod topological_sort_dfs_cycle_graph_as_map;

const EPSILON: f64 = 1e-10;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

impl PartialOrd for DirectedEdge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DirectedEdge {
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

impl fmt::Display for DirectedEdge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {} {:.2} ", self.v, self.u, self.weight)
    }
}

#[derive(Debug, Clone)]
pub struct Digraph {
    v: usize,
    e: usize,
    adj: Vec<VecDeque<usize>>,
}

impl Digraph {
    pub fn new(v: usize) -> Self {
        Digraph {
            v,
            e: 0,
            adj: vec![VecDeque::new(); v],
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

        let mut graph = Digraph::new(v);

        let _ = iterator.next();

        for line in iterator {
            let line: String = line?;
            let edge_parts: Vec<&str> = line.split_whitespace().collect();
            let from: usize = edge_parts[0].parse().map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;
            let to: usize = edge_parts[1].parse().map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;
            graph.add_edge(from, to);
        }

        Ok(graph)
    }


    pub fn add_edge(&mut self, v: usize, u: usize) {
        self.adj[v].push_front(u);
        self.e += 1;
    }

    pub fn adj(&self, v: usize) -> impl Iterator<Item = &usize> {
        self.adj[v].iter()
    }
}

impl fmt::Display for Digraph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} {}", self.v, self.e)?;
        for v in 0..self.v {
            write!(f, "{}: ", v)?;
            for u in &self.adj[v] {
                write!(f, "{} ", u)?;
            }
            writeln!(f)?;
        }

        Ok(())
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

    pub fn edges(&self) -> Vec<Rc<Edge>> {
        let mut res = vec![];
        for v in 0..self.v {
            let mut self_loops = 0;
            for edge in &self.adj[v] {
                if edge.other(v) > v {
                    res.push(Rc::clone(edge));
                } else if edge.other(v) == v {
                    if self_loops % 2 == 0 {
                        res.push(Rc::clone(edge));
                    }
                    self_loops += 1;
                }
                // res.push(Rc::clone(edge));
            }
        }

        res
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

#[derive(Debug, Clone)]
pub struct Node {
    pub val: i32,
    pub neighbors: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(val: i32) -> Self {
        Node {
            val,
            neighbors: vec![],
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let neighbors_str = self.neighbors.iter().map(|x| x.borrow().val).join(", ");
        writeln!(f, "{} [{}]", self.val, neighbors_str)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct EdgeT<T: Clone + PartialEq> {
    v: T,
    u: T,
    weight: f64
}

impl <T: Clone + PartialEq> EdgeT<T> {
    pub fn new(v: T, u: T, weight: f64) -> Self {
        EdgeT {
            v,
            u,
            weight
        }
    }

    pub fn from(&self) -> &T {
        &self.v
    }

    pub fn to(&self) -> &T {
        &self.u
    }

    pub fn weight(&self) -> f64 {
        self.weight
    }

    pub fn other(&self, v: &T) -> &T {
        if &self.v == v {
            &self.u
        } else if &self.u == v {
            &self.v
        } else {
            panic!("Unreachable")
        }
    }
}

impl<T: Clone + PartialEq> Eq for EdgeT<T> {}

impl <T: Clone + PartialEq> PartialEq for EdgeT<T> {
    fn eq(&self, other: &Self) -> bool {
        self.v.eq(&other.v) && self.u.eq(&other.u) && nearly_equal(self.weight, other.weight, EPSILON)
    }
}

impl <T: Clone + PartialEq> PartialOrd for EdgeT<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl <T: Clone + PartialEq> Ord for EdgeT<T> {
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

impl Hash for EdgeT<String> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.v.hash(state);
        self.u.hash(state);
        self.weight.to_bits().hash(state);
    }
}

impl <T: Display + Clone + PartialEq> Display for EdgeT<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {} {:.2} ", self.v, self.u, self.weight)
    }
}

pub fn graph_from_file(path: PathBuf) -> Result<HashMap<String, Vec<Rc<EdgeT<String>>>>, Error> {

    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut iterator = reader.lines();
    let _ = iterator.next(); // read amount of vertices
    let _ = iterator.next(); // read amount of edges

    let mut graph = HashMap::new();

    for line in iterator {
        let line: String = line?;
        let edge_parts: Vec<&str> = line.split_whitespace().collect();
        let from: String = edge_parts[0].parse().map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;
        let to: String = edge_parts[1].parse().map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;
        let weight: f64 = edge_parts[2].parse().map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;
        let edge = Rc::new(EdgeT::new(from.clone(), to, weight));

        let v = graph.entry(from).or_insert_with(Vec::new);
        v.push(edge);
    }


    Ok(graph)
}

pub fn graph_to_string(graph: &HashMap<String, Vec<Rc<EdgeT<String>>>>) -> String {
    let mut res = String::new();
    res.push_str(format!("{} ", graph.len()).as_str());

    let mut edges = HashSet::new();

    let mut sub_res = String::new();
    for (v, lst) in graph.iter() {
        sub_res.push_str(format!("{}: ", v).as_str());
        edges.extend(lst.iter().map(|x| Rc::as_ptr(x)));

        for edge in lst.iter() {
            sub_res.push_str(format!("{}  ", edge).as_str());
        }
        sub_res.push_str("\n");
    }
    res.push_str(format!("{}\n", edges.len()).as_str());
    res.push_str(sub_res.as_str());
    res
}

/*
    6 9
    2: 0 -> 2 8.00   1 -> 2 3.00   2 -> 3 4.00   2 -> 4 3.00
    3: 1 -> 3 6.00   2 -> 3 4.00   3 -> 4 2.00   3 -> 5 5.00
    5: 3 -> 5 5.00   4 -> 5 2.00
    4: 2 -> 4 3.00   3 -> 4 2.00   4 -> 5 2.00
    0: 0 -> 1 7.00   0 -> 2 8.00
    1: 0 -> 1 7.00   1 -> 2 3.00   1 -> 3 6.00
*/
pub fn create_graph() -> HashMap<String, Vec<Rc<EdgeT<String>>>> {
    let mut graph = HashMap::new();

    let edge01 = Rc::new(EdgeT::new("0".to_owned(), "1".to_owned(), 7.0));
    let edge02 = Rc::new(EdgeT::new("0".to_owned(), "2".to_owned(), 8.0));

    let edge12 = Rc::new(EdgeT::new("1".to_owned(), "2".to_owned(), 3.0));
    let edge13 = Rc::new(EdgeT::new("1".to_owned(), "3".to_owned(), 6.0));

    let edge23 = Rc::new(EdgeT::new("2".to_owned(), "3".to_owned(), 4.0));
    let edge24 = Rc::new(EdgeT::new("2".to_owned(), "4".to_owned(), 3.0));

    let edge34 = Rc::new(EdgeT::new("3".to_owned(), "4".to_owned(), 2.0));
    let edge35 = Rc::new(EdgeT::new("3".to_owned(), "5".to_owned(), 5.0));

    let edge45 = Rc::new(EdgeT::new("4".to_owned(), "5".to_owned(), 2.0));

    graph.insert(String::from("0"), vec![Rc::clone(&edge01), Rc::clone(&edge02)]);
    graph.insert(String::from("1"), vec![Rc::clone(&edge01), Rc::clone(&edge12), Rc::clone(&edge13)]);
    graph.insert(String::from("2"), vec![Rc::clone(&edge02), Rc::clone(&edge12), Rc::clone(&edge23), Rc::clone(&edge24)]);
    graph.insert(String::from("3"), vec![Rc::clone(&edge13), Rc::clone(&edge23), Rc::clone(&edge34), Rc::clone(&edge35)]);
    graph.insert(String::from("4"), vec![Rc::clone(&edge24), Rc::clone(&edge34), Rc::clone(&edge45)]);
    graph.insert(String::from("5"), vec![Rc::clone(&edge35), Rc::clone(&edge45)]);

    graph
}

/*
    7 10
    0: 0->1 2.0  0->2 3.0  0->3 7.0
    1: 0->1 2.0  1->2 6.0  1->6 3.0
    2: 0->2 3.0  1->2 6.0  2->4 1.0  2->5 8.0
    3: 0->3 7.0  3->4 5.0
    4: 2->4 1.0  3->4 5.0  4->5 4.0
    5: 2->5 8.0  4->5 4.0  5->6 2.0
    6: 1->6 3.0  5->6 2.0
*/
pub fn create_graph1() -> HashMap<String, Vec<Rc<EdgeT<String>>>> {
    let mut graph = HashMap::new();

    let edge01 = Rc::new(EdgeT::new("0".to_owned(), "1".to_owned(), 2.0));
    let edge02 = Rc::new(EdgeT::new("0".to_owned(), "2".to_owned(), 3.0));
    let edge03 = Rc::new(EdgeT::new("0".to_owned(), "3".to_owned(), 7.0));

    let edge12 = Rc::new(EdgeT::new("1".to_owned(), "2".to_owned(), 6.0));
    let edge16 = Rc::new(EdgeT::new("1".to_owned(), "6".to_owned(), 3.0));

    let edge24 = Rc::new(EdgeT::new("2".to_owned(), "4".to_owned(), 1.0));
    let edge25 = Rc::new(EdgeT::new("2".to_owned(), "5".to_owned(), 8.0));

    let edge34 = Rc::new(EdgeT::new("3".to_owned(), "4".to_owned(), 5.0));

    let edge45 = Rc::new(EdgeT::new("4".to_owned(), "5".to_owned(), 4.0));

    let edge56 = Rc::new(EdgeT::new("5".to_owned(), "6".to_owned(), 2.0));

    graph.insert(String::from("0"), vec![Rc::clone(&edge01), Rc::clone(&edge02), Rc::clone(&edge03)]);
    graph.insert(String::from("1"), vec![Rc::clone(&edge01), Rc::clone(&edge12), Rc::clone(&edge16)]);
    graph.insert(String::from("2"), vec![Rc::clone(&edge02), Rc::clone(&edge12), Rc::clone(&edge24), Rc::clone(&edge25)]);
    graph.insert(String::from("3"), vec![Rc::clone(&edge03), Rc::clone(&edge34)]);
    graph.insert(String::from("4"), vec![Rc::clone(&edge24), Rc::clone(&edge34), Rc::clone(&edge45)]);
    graph.insert(String::from("5"), vec![Rc::clone(&edge25), Rc::clone(&edge45), Rc::clone(&edge56)]);
    graph.insert(String::from("6"), vec![Rc::clone(&edge16), Rc::clone(&edge56)]);

    graph
}

/*
    6 5
    0: 0-1 7.0
    1: 1-2 3.0  0-1 7.0
    2: 1-2 3.0  2-4 3.0
    3: 3-4 2.0
    4: 3-4 2.0  4-5 2.0  2-4 3.0
    5: 4-5 2.0
*/
pub fn exp_graph() -> HashMap<String, Vec<Rc<EdgeT<String>>>> {
    let mut exp_graph = HashMap::new();
    let edge01 = Rc::new(EdgeT::new("0".to_owned(), "1".to_owned(), 7.0));
    let edge12 = Rc::new(EdgeT::new("1".to_owned(), "2".to_owned(), 3.0));
    let edge24 = Rc::new(EdgeT::new("2".to_owned(), "4".to_owned(), 3.0));
    let edge34 = Rc::new(EdgeT::new("3".to_owned(), "4".to_owned(), 2.0));
    let edge45 = Rc::new(EdgeT::new("4".to_owned(), "5".to_owned(), 2.0));
    exp_graph.insert(String::from("0"), vec![Rc::clone(&edge01)]);
    exp_graph.insert(String::from("1"), vec![Rc::clone(&edge12), Rc::clone(&edge01)]);
    exp_graph.insert(String::from("2"), vec![Rc::clone(&edge12), Rc::clone(&edge24)]);
    exp_graph.insert(String::from("3"), vec![Rc::clone(&edge34)]);
    exp_graph.insert(String::from("4"), vec![Rc::clone(&edge34), Rc::clone(&edge45), Rc::clone(&edge24)]);
    exp_graph.insert(String::from("5"), vec![Rc::clone(&edge45)]);
    exp_graph
}

/*
    7 6
    0: 0-1 2.0  0-2 3.0
    1: 0-1 2.0  1-6 3.0
    2: 2-4 1.0  0-2 3.0
    3: 3-4 5.0
    4: 2-4 1.0  3-4 5.0
    5: 5-6 2.0
    6: 5-6 2.0  1-6 3.0
*/
pub fn exp_graph1() -> HashMap<String, Vec<Rc<EdgeT<String>>>> {
    let mut exp_graph = HashMap::new();
    let edge01 = Rc::new(EdgeT::new("0".to_owned(), "1".to_owned(), 2.0));
    let edge02 = Rc::new(EdgeT::new("0".to_owned(), "2".to_owned(), 3.0));
    let edge16 = Rc::new(EdgeT::new("1".to_owned(), "6".to_owned(), 3.0));
    let edge24 = Rc::new(EdgeT::new("2".to_owned(), "4".to_owned(), 1.0));
    let edge34 = Rc::new(EdgeT::new("3".to_owned(), "4".to_owned(), 5.0));
    let edge56 = Rc::new(EdgeT::new("5".to_owned(), "6".to_owned(), 2.0));

    exp_graph.insert(String::from("0"), vec![Rc::clone(&edge01), Rc::clone(&edge02)]);
    exp_graph.insert(String::from("1"), vec![Rc::clone(&edge01), Rc::clone(&edge16)]);
    exp_graph.insert(String::from("2"), vec![Rc::clone(&edge02), Rc::clone(&edge24)]);
    exp_graph.insert(String::from("3"), vec![Rc::clone(&edge34)]);
    exp_graph.insert(String::from("4"), vec![Rc::clone(&edge24), Rc::clone(&edge34)]);
    exp_graph.insert(String::from("5"), vec![Rc::clone(&edge56)]);
    exp_graph.insert(String::from("6"), vec![Rc::clone(&edge16), Rc::clone(&edge56)]);
    exp_graph
}



pub fn compare(graph1: &EdgeWeightedGraph, graph2: &EdgeWeightedGraph) -> bool {
    if graph1.v != graph2.v {
        println!("graph1.v != graph2.v");
        return false;
    }
    if graph1.e != graph2.e {
        println!("graph1.e != graph2.e");
        return false;
    }
    for v in 0..graph1.v {
        let vec1 = graph1.adj[v].iter().sorted().collect::<Vec<_>>();
        let vec2 = graph2.adj[v].iter().sorted().collect::<Vec<_>>();
        if vec1 != vec2 {
            println!("graph1 {} {:?} != graph2 {} {:?}", v, graph1.adj[v], v, graph2.adj[v]);
            return false;
        }
    }

    true
}

pub fn compare_as_map(graph1: &HashMap<String, Vec<Rc<EdgeT<String>>>>, graph2: &HashMap<String, Vec<Rc<EdgeT<String>>>>) -> bool {
    if graph1.len() != graph2.len() {
        println!("graph1.v != graph2.v");
        return false;
    }

    for v in graph1.keys() {
        let vec1 = graph1.get(v);
        let vec2 = graph2.get(v);
        if !compare_vectors(vec1, vec2) {
            println!("graph1 {} {:?} != graph2 {} {:?}", v, vec1, v, vec2);
            return false;
        }
    }

    true
}

fn compare_vectors(v1: Option<&Vec<Rc<EdgeT<String>>>>, v2: Option<&Vec<Rc<EdgeT<String>>>>) -> bool {
    match (v1, v2) {
        (Some(v1), Some(v2)) => {
            if v1.len() != v2.len() {
                return false;
            }
            let mp = create_map(v1);
            for edge2 in v2.iter() {
                let key = format!("{}_{}", edge2.v, edge2.u);
                match mp.get(&key) {
                    Some(edge1) =>
                        if !edge1.eq(edge2) {
                            println!("edge {:?} != {:?}", edge1, edge2);
                            return false;
                        }
                    None => {
                        return false;
                    }
                }
            }
            true
        }
        (_, _) => false
    }
}

fn create_map(vec: &Vec<Rc<EdgeT<String>>>) -> HashMap<String, Rc<EdgeT<String>>> {
    vec.into_iter().map(|x| {
        let key = format!("{}_{}", x.v, x.u);
        (key, Rc::clone(&x))
    }).collect()
}