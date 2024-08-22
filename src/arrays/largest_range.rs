use std::collections::HashMap;


// O(n * log(n)) time | O(1) space
pub fn largest_range(arr: &[i32]) -> (i32, i32) {

    let mut copy_arr = arr.to_vec();
    copy_arr.sort();

    let mut start = copy_arr[0];
    let mut curr_start = copy_arr[0];
    let mut end = copy_arr[0];
    let mut max = 0;
    let mut curr_max = 0;

    for i in 1..copy_arr.len() {
        if copy_arr[i - 1] + 1 == copy_arr[i] {
            curr_max += 1;            
        } else {
            curr_max = 1;
            curr_start = copy_arr[i];
        }

        if max < curr_max {
            max = curr_max;
            start = curr_start;
            end = copy_arr[i];
        }
    }    

    (start, end)
}

// O(n) time | O(n) space
pub fn largest_range1(arr: &[i32]) -> (i32, i32) {

    let mut map = HashMap::new();

    for v in arr {
        map.insert(*v, true);
    }

    let mut best_range = (-1, -1);
    let mut max_length = 0;

    for v in arr {

        if !map.get(v).unwrap_or(&false) {
            continue;
        }

        let mut length = 1;
        let mut l = v - 1;

        while *map.get(&l).unwrap_or(&false) {
            length += 1;
            map.insert(l, false);
            l -= 1;
        }

        let mut r = v + 1;
        while *map.get(&r).unwrap_or(&false) {
            length += 1;
            map.insert(r, false);
            r += 1;
        }

        if max_length < length {
            max_length = length;
            best_range = (l + 1, r - 1);
        }
    }
    best_range
}

#[cfg(test)]
mod tests {

    use super::largest_range;
    use super::largest_range1;

    const ARR: &[i32] = &[1, 11, 3, 0, 15, 5, 2, 4, 10, 7, 12, 6];

    #[test]
    fn it_largest_range() {
        assert_eq!(largest_range(ARR), (0, 7));
    }

    #[test]
    fn it_largest_range1() {
        let res = largest_range1(ARR);
        println!("{res:?}");
        assert_eq!(res, (0, 7));
    }

}