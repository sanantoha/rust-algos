use std::collections::HashMap;

// O(b * r) time | O(b * r) space
pub fn apartment_hunting(blocks: Vec<HashMap<String, bool>>, reqs: Vec<String>) -> Result<usize, String> {
    if blocks.is_empty() || reqs.is_empty() {
        return Err("input data is empty".to_string());
    }

    let mut distances = init_distance(&blocks, &reqs);

    for i in 0..blocks.len() {
        for req in reqs.iter() {
            if blocks[i].contains_key(req) {
                let mut d = distances[i].get(req).copied().unwrap_or(i32::MAX);
                if blocks[i].get(req).copied().unwrap_or_default() {
                    d = 0;
                } else {
                    let prev_distance = if i > 0 {
                        distances[i - 1].get(req).copied().unwrap_or(i32::MAX)
                    } else {
                        i32::MAX
                    };
                    if prev_distance != i32::MAX {
                        d = d.min(prev_distance + 1);
                    }
                }
                distances[i].insert(req.to_owned(), d);
            }
        }
    }

    for i in (0..blocks.len()).rev() {
        for req in reqs.iter() {
            if blocks[i].contains_key(req) {
                let mut d = distances[i].get(req).copied().unwrap_or(i32::MAX);
                let prev_distance = if i < blocks.len() - 1 {
                    distances[i + 1].get(req).copied().unwrap_or(i32::MAX)
                } else {
                    i32::MAX
                };
                if prev_distance != i32::MAX {
                    d = d.min(prev_distance + 1);
                }
                distances[i].insert(req.to_owned(), d);
            }
        }
    }

    let mut min_distance_value = i32::MAX;
    let mut min_distance_idx = 0;

    for i in 0..distances.len() {
        let distance = &distances[i];
        let mut max_distance = i32::MIN;
        for (_, v) in distance {
            max_distance = max_distance.max(*v);
        }
        if min_distance_value > max_distance {
            min_distance_value = max_distance;
            min_distance_idx = i;
        }
    }
    println!("{:?}", distances);
    println!("{}", min_distance_idx);
    Ok(min_distance_idx)
}

fn init_distance(blocks: &Vec<HashMap<String, bool>>, reqs: &Vec<String>) -> Vec<HashMap<String, i32>> {
    let mut distances = vec![];
    for i in 0..blocks.len() {
        let mut map = HashMap::new();
        for req in reqs.iter() {
            if blocks[i].contains_key(req) {
                map.insert(req.to_owned(), i32::MAX);
            }
        }
        distances.push(map);
    }
    distances
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::apartment_hunting;

    #[test]
    fn test_apartment_hunting() {
        let blocks: Vec<HashMap<String, bool>> = vec![
            {
                let mut map = HashMap::new();
                map.insert("gym".to_string(), false);
                map.insert("school".to_string(), true);
                map.insert("store".to_string(), false);
                map
            },
            {
                let mut map = HashMap::new();
                map.insert("gym".to_string(), true);
                map.insert("school".to_string(), false);
                map.insert("store".to_string(), false);
                map
            },
            {
                let mut map = HashMap::new();
                map.insert("gym".to_string(), true);
                map.insert("school".to_string(), true);
                map.insert("store".to_string(), false);
                map
            },
            {
                let mut map = HashMap::new();
                map.insert("gym".to_string(), false);
                map.insert("school".to_string(), true);
                map.insert("store".to_string(), false);
                map
            },
            {
                let mut map = HashMap::new();
                map.insert("gym".to_string(), false);
                map.insert("school".to_string(), true);
                map.insert("store".to_string(), true);
                map
            },
        ];
        let reqs: Vec<String> = vec!["gym".to_string(), "school".to_string(), "store".to_string()];

        assert_eq!(apartment_hunting(blocks, reqs), Ok(3));
    }
}