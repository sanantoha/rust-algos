use std::collections::VecDeque;

// O(E + V) time | O(V) space
pub fn can_finish(num_courses: usize, prerequisites: &[&[usize]]) -> bool {
    if num_courses == 0 || prerequisites.is_empty() {
        return true;
    }

    let adj_list = create_adj_list(num_courses, prerequisites);

    let mut visited = vec![0; num_courses];

    for v in 0..num_courses {
        if visited[v] == 0 {
            if can_not_finish_dfs(&adj_list, &mut visited, v) {
                return false;
            }
        }
    }

    true
}

fn can_not_finish_dfs(adj_list: &Vec<Vec<usize>>, visited: &mut Vec<usize>, v: usize) -> bool {
    visited[v] = 1;

    for &u in &adj_list[v] {
        if visited[u] == 1 {
            return true;
        }
        if visited[u] == 0 {
            if can_not_finish_dfs(adj_list, visited, u) {
                return true;
            }
        }
    }

    visited[v] = 2;
    false
}

fn create_adj_list(num_courses: usize, prerequisites: &[&[usize]]) -> Vec<Vec<usize>> {
    let mut adj_list = vec![vec![]; num_courses];

    for &edge in prerequisites {
        adj_list[edge[0]].push(edge[1]);
    }

    adj_list
}

// O(E + V) time | O(V) space
pub fn can_finish1(num_courses: usize, prerequisites: &[&[usize]]) -> bool {
    if num_courses == 0 || prerequisites.is_empty() {
        return true;
    }

    let adj_list = create_adj_list(num_courses, prerequisites);
    let mut cnt = vec![0; num_courses];

    for v in 0..num_courses {
        for &u in &adj_list[v] {
            cnt[u] += 1;
        }
    }

    let mut is_circle = true;
    let mut queue = VecDeque::new();

    for v in 0..num_courses {
        if cnt[v] == 0 {
            is_circle = false;
            queue.push_back(v);
        }
    }

    if is_circle {
        return false;
    }

    let mut idx = 0;

    while let Some(v) = queue.pop_front() {

        idx += 1;

        for &u in &adj_list[v] {
            cnt[u] -= 1;

            if cnt[u] == 0 {
                queue.push_back(u);
            }
        }
    }

    if idx != num_courses {
        return false;
    }

    true
}


#[cfg(test)]
mod tests {

    use crate::graph::course_schedule::{can_finish, can_finish1};

    #[test]
    fn test_can_finish_case1() {
        let prerequisites: &[&[usize]] = &[];
        assert!(can_finish(1, &prerequisites));
    }

    #[test]
    fn test_can_finish_case2() {
        let prerequisites: &[&[usize]] = &[&[1, 0]];
        assert!(can_finish(2, &prerequisites));
    }

    #[test]
    fn test_can_finish_case3() {
        let prerequisites: &[&[usize]] = &[&[1, 0], &[0, 1]];
        assert!(!can_finish(2, &prerequisites));
    }

    #[test]
    fn test_can_finish_case4() {
        let prerequisites: &[&[usize]] = &[&[1, 0], &[2, 1], &[3, 2], &[0, 3]];
        assert!(!can_finish(4, &prerequisites));
    }

    #[test]
    fn test_can_finish_case5() {
        let prerequisites: &[&[usize]] = &[&[1, 0], &[2, 1], &[3, 2]];
        assert!(can_finish(4, &prerequisites));
    }

    #[test]
    fn test_can_finish1_case1() {
        let prerequisites: &[&[usize]] = &[];
        assert!(can_finish1(1, &prerequisites));
    }

    #[test]
    fn test_can_finish1_case2() {
        let prerequisites: &[&[usize]] = &[&[1, 0]];
        assert!(can_finish1(2, &prerequisites));
    }

    #[test]
    fn test_can_finish1_case3() {
        let prerequisites: &[&[usize]] = &[&[1, 0], &[0, 1]];
        assert!(!can_finish1(2, &prerequisites));
    }

    #[test]
    fn test_can_finish1_case4() {
        let prerequisites: &[&[usize]] = &[&[1, 0], &[2, 1], &[3, 2], &[0, 3]];
        assert!(!can_finish1(4, &prerequisites));
    }

    #[test]
    fn test_can_finish1_case5() {
        let prerequisites: &[&[usize]] = &[&[1, 0], &[2, 1], &[3, 2]];
        assert!(can_finish1(4, &prerequisites));
    }
}