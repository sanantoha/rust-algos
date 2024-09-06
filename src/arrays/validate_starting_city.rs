use std::i32;


// O(n ^ 2) time | O(1) space
pub fn valid_starting_city(distances: &[i32], fuel: &[i32], mpg: i32) -> Option<usize> {
    if distances.is_empty() || fuel.is_empty() || mpg <= 0 {
        return None;
    }

    for city in 0..distances.len() {
        let mut remaining_distance = 0;
        for i in city..(city + distances.len()) {
            let idx = i % distances.len();
            remaining_distance += fuel[idx] * mpg - distances[idx];
            if remaining_distance < 0 {
                break;
            }
        }
        if remaining_distance >= 0 {
            return Some(city);
        }
    }

    None
}

// O(n) time | O(1) space
pub fn valid_starting_city1(distances: &[i32], fuel: &[i32], mpg: i32) -> Option<usize> {
    if distances.is_empty() || fuel.is_empty() || mpg <= 0 {
        return None;
    }

    let mut res_city = 0;
    let mut min_remain_distance = i32::MAX;
    let mut remain_distance = 0;

    for city in 1..distances.len() {
        remain_distance += fuel[city - 1] * mpg - distances[city - 1];

        if remain_distance < min_remain_distance {
            min_remain_distance = remain_distance;
            res_city = city;
        }
    }
    
    Some(res_city)
}

#[cfg(test)]
mod tests {

    use super::valid_starting_city;
    use super::valid_starting_city1;

    const DISTANCES: &[i32] = &[5, 25, 15, 10, 15];
    const FUEL: &[i32] = &[1, 2, 1, 0, 3];
    const MPG: i32 = 10;
    const EXP: usize = 4;

    #[test]
    fn it_valid_starting_city() {
        assert_eq!(valid_starting_city(DISTANCES, FUEL, MPG), Some(EXP));
    }

    #[test]
    fn it_valid_starting_city1() {
        assert_eq!(valid_starting_city1(DISTANCES, FUEL, MPG), Some(EXP));
    }
}