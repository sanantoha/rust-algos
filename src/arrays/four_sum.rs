use std::collections::HashMap;

// O(n ^ 4) time | O(n ^ 2) space
pub fn four_sum(arr: &[i32], target: i32) -> Vec<[i32;4]> {
    let mut res = vec![];

    for i in 0..arr.len() {
        for j in (i + 1)..arr.len() {
            for k in (j + 1)..arr.len() {
                for z in (k + 1)..arr.len() {
                    if target == arr[i] + arr[j] + arr[k] + arr[z] {
                        res.push([arr[i], arr[j], arr[k], arr[z]]);
                    }
                }
            }
        }
    }
    res
}

// O(n ^ 3) time | O(n ^ 2) space
pub fn four_sum1(arr: &[i32], target: i32) -> Vec<[i32;4]> {

    let mut res = vec![];

    let mut map: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();

    for i in 1..arr.len() {

        for j in (i + 1)..arr.len() {
            let compensate = target - arr[i] - arr[j];
            if let Some(lst) = map.get(&compensate) {
                for (fst, snd) in lst {
                    res.push([arr[i], arr[j], *fst, *snd]);
                }                
            }
        }

        for j in (0..=i).rev() {
            let key = arr[i] + arr[j];
            let lst = map.entry(key).or_default();
            lst.push((arr[i], arr[j]));
        }

    }

    res
}

#[cfg(test)]
mod tests {
    use crate::arrays::four_sum::*;


    const ARR: &[i32] = &[7, 6, 4, -1, 1, 2];

    #[test]
    fn test_four_sum() {
        assert_eq!(four_sum(ARR, 16), vec![[7, 6, 4, -1], [7, 6, 1, 2]])
    }

    #[test]
    fn test_four_sum1() {
        assert_eq!(four_sum1(ARR, 16), vec![[4, -1, 6, 7], [1, 2, 6, 7]])
    }
}