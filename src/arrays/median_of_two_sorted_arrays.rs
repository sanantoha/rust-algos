
// O(a1 + a2) time | O(a1 + a2) space
pub fn median(arr1: &[i32], arr2: &[i32]) -> f64 {
    if arr1.is_empty() {
        return calc_median(arr2);
    }
    if arr2.is_empty() {
        return calc_median(arr1);
    }

    let arr = merge(arr1, arr2);
    
    calc_median(&arr)
}

fn calc_median(arr: &[i32]) -> f64 {
    if arr.is_empty() {
        return f64::NAN;
    }
    let n = arr.len();
    let h = n / 2;
    if n % 2 == 0 {        
        ((arr[h] + arr[h - 1]) as f64) / 2.0
    } else {
        arr[h] as f64
    }
}

fn merge(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut res = vec![];

    let mut i = 0;
    let mut j = 0;

    while i < arr1.len() && j < arr2.len() {
        if arr1[i] <= arr2[j] {
            res.push(arr1[i]);
            i += 1;
        } else {
            res.push(arr2[j]);
            j += 1;
        }
    }

    while i < arr1.len() {
        res.push(arr1[i]);
        i += 1;
    }

    while j < arr2.len() {
        res.push(arr2[j]);
        j += 1;
    }

    res
}

// O(log(min(a1, a2))) time | O(1) space
pub fn median1(arr1: &[i32], arr2: &[i32]) -> f64 {
    if arr1.is_empty() {
        return calc_median(arr2);
    }
    if arr2.is_empty() {
        return calc_median(arr1);
    }

    let len1 = arr1.len();
    let len2 = arr2.len();

    if len1 > len2 {
        return median1(arr2, arr1);
    }

    let mut l = 0;
    let mut r = len1;

    while l <= r {
        let mid1 = l + (r - l) / 2;
        let mid2 = (len1 + len2 + 1) / 2 - mid1;

        let max_left_1 = if mid1 == 0 { i32::MIN } else { arr1[mid1 - 1] } ;
        let min_right_1 = if mid1 == len1 { i32::MAX } else { arr1[mid1] };
        let max_left_2 = if mid2 == 0 { i32::MIN } else { arr2[mid2 - 1] };
        let min_right_2 = if mid2 == len2 { i32::MAX } else { arr2[mid2] };

        if max_left_1 <= min_right_2 && max_left_2 <= min_right_1 {
            let len = len1 + len2;
            if len % 2 == 0 {
                return ((max_left_1.max(max_left_2) + min_right_1.min(min_right_2)) as f64) / 2.0;
            } else {
                return max_left_1.max(max_left_2) as f64;
            }
        } else if max_left_1 > min_right_2 {
            r = mid1 - 1;
        } else {
            l = mid1 + 1;
        }
    }
    
    -1.0
}

#[cfg(test)]
mod tests {
    use super::{median, median1};

    const ARR1: &[i32] = &[1,2,3,4,5,6];
    const ARR2: &[i32] = &[7,8,9,10,11,12];
    const ARR3: &[i32] = &[1,2,3,4,5,6,13];

    const EXP_RES: f64 = 6.5;
    const EXP_RES1: f64 = 7.0;

    #[test]
    fn it_median() {
        assert_eq!(median(ARR1, ARR2), EXP_RES);
    }

    #[test]
    fn it_median_one() {
        assert_eq!(median(ARR3, ARR2), EXP_RES1);
    }

    #[test]
    fn it_median1() {        
        assert_eq!(median1(ARR1, ARR2), EXP_RES);
    }

    #[test]
    fn it_median1_one() {
        assert_eq!(median1(ARR3, ARR2), EXP_RES1);
    }
}