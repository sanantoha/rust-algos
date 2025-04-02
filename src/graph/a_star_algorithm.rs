use std::cell::RefCell;
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use std::rc::Rc;

// O(w * h * log(w * h)) time | O(w * h) space
pub fn a_star_algorithm(start_row: usize, start_col: usize, end_row: usize, end_col: usize, graph: &[&[i32]]) -> Vec<Vec<usize>> {
    let nodes = init_nodes(graph);
    let end_node = Rc::clone(&nodes[end_row][end_col]);
    let start_node = Rc::clone(&nodes[start_row][start_col]);
    start_node.borrow_mut().distance_from_start = 0;
    start_node.borrow_mut().distance_to_end = calculate_manhattan_distance(Rc::clone(&start_node), Rc::clone(&end_node));

    let mut heap = BinaryHeap::new();
    heap.push(Reverse(Rc::clone(&start_node)));

    while let Some(Reverse(node)) = heap.pop() {

        if Rc::ptr_eq(&node, &end_node) {
            break;
        }

        for neighbor in get_neighbors(&nodes, Rc::clone(&node)) {
            if neighbor.borrow().val == 1 {
                continue;
            }
            let tentative_distance = node.borrow().distance_from_start + 1;
            if tentative_distance >= neighbor.borrow().distance_from_start {
                continue;
            }
            neighbor.borrow_mut().distance_from_start = tentative_distance;
            neighbor.borrow_mut().distance_to_end = tentative_distance + calculate_manhattan_distance(Rc::clone(&neighbor), Rc::clone(&end_node));
            neighbor.borrow_mut().came_from = Some(Rc::clone(&node));

            remove(&mut heap, Rc::clone(&neighbor));
            heap.push(Reverse(Rc::clone(&neighbor)));
        }
    }

    reconstruct_path(Rc::clone(&end_node))
}

fn reconstruct_path(end_node: Rc<RefCell<Node>>) -> Vec<Vec<usize>> {
    let mut res = vec![];

    let mut node = Some(Rc::clone(&end_node));

    while let Some(n) = node.take() {
        res.push(vec![n.borrow().row, n.borrow().col]);

        node = n.borrow().came_from.as_ref().map(Rc::clone);
    }

    res.reverse();
    res
}

fn remove(heap: &mut BinaryHeap<Reverse<Rc<RefCell<Node>>>>, node: Rc<RefCell<Node>>) {

    let mut tmp_heap = BinaryHeap::new();

    while let Some(Reverse(neighbor)) = heap.pop() {
        if !Rc::ptr_eq(&node, &neighbor) {
            tmp_heap.push(Reverse(Rc::clone(&neighbor)));
        }
    }

    while let Some(Reverse(neighbor)) = tmp_heap.pop() {
        heap.push(Reverse(Rc::clone(&neighbor)));
    }
}

fn get_neighbors(nodes: &Vec<Vec<Rc<RefCell<Node>>>>, node: Rc<RefCell<Node>>) -> Vec<Rc<RefCell<Node>>> {
    let mut neighbors = vec![];

    let row = node.borrow().row;
    let col = node.borrow().col;

    if row > 0 {
        neighbors.push(Rc::clone(&nodes[row - 1][col]));
    }
    if col > 0 {
        neighbors.push(Rc::clone(&nodes[row][col - 1]));
    }
    if row + 1 < nodes.len() {
        neighbors.push(Rc::clone(&nodes[row + 1][col]));
    }
    if col + 1 < nodes[row].len() {
        neighbors.push(Rc::clone(&nodes[row][col + 1]));
    }

    neighbors
}

fn calculate_manhattan_distance(start_node: Rc<RefCell<Node>>, end_node: Rc<RefCell<Node>>) -> usize {
    start_node.borrow().row.abs_diff(end_node.borrow().row) + start_node.borrow().col.abs_diff(end_node.borrow().col)
}

fn init_nodes(graph: &[&[i32]]) -> Vec<Vec<Rc<RefCell<Node>>>> {
    let mut nodes = vec![];
    for i in 0..graph.len() {
        nodes.push(vec![]);
        for j in 0..graph[i].len() {
            let node = Node::new(i, j, graph[i][j]);
            nodes[i].push(Rc::new(RefCell::new(node)));
        }
    }
    nodes
}

#[derive(Debug, Eq, PartialEq)]
struct Node {
    row: usize,
    col: usize,
    val: i32,
    distance_from_start: usize,
    distance_to_end: usize,
    came_from: Option<Rc<RefCell<Node>>>
}

impl Node {
    fn new(row: usize, col: usize, val: i32) -> Self {
        Node {
            row,
            col,
            val,
            distance_from_start: usize::MAX,
            distance_to_end: usize::MAX,
            came_from: None
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance_to_end.cmp(&other.distance_to_end)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::a_star_algorithm::a_star_algorithm;

    #[test]
    fn test_a_star_algorithm() {
        let start_row = 0;
        let start_col = 1;
        let end_row = 4;
        let end_col = 3;

        let graph: &[&[i32]] = &[
            &[0, 0, 0, 0, 0],
            &[0, 1, 1, 1, 0],
            &[0, 0, 0, 0, 0],
            &[1, 0, 1, 1, 1],
            &[0, 0, 0, 0, 0],
        ];

        let expected = vec![
            vec![0, 1],
            vec![0, 0],
            vec![1, 0],
            vec![2, 0],
            vec![2, 1],
            vec![3, 1],
            vec![4, 1],
            vec![4, 2],
            vec![4, 3],
        ];

        let res = a_star_algorithm(start_row, start_col, end_row, end_col, graph);
        println!("{:?}", res);
        assert_eq!(res, expected);
    }
}