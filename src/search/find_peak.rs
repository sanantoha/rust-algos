
// O(log(n)) time | O(v) space
pub fn find_peak(arr: &[i32]) -> Option<usize> {
    let mut l = 0;
    let mut r = arr.len() - 1;

    while l < r {
        let mid = l + (r - l) / 2;

        if arr[mid] < arr[mid + 1] {
            l = mid + 1;
        } else {
            r = mid;
        }

    }

    Some(l)
}

#[cfg(test)]
mod tests {
    use crate::search::find_peak::find_peak;

    #[test]
    fn test_find_peak() {
        let res = find_peak(&[1,2,1,3,4,5,6,5,4,3,2,1]);
        println!("{:?}", res);

        assert_eq!(res, Some(6));
    }
}