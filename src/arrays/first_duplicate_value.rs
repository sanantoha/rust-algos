
// O(n) time | O(1) space
pub fn first_duplicate_value(arr: &mut [i32]) -> i32 {
    let size = arr.len();

    for i in 0..size {
        let v = arr[i];
        let idx = v.unsigned_abs() as usize;
        if arr[idx - 1] < 0 {
            return idx as i32;
        }
        arr[idx - 1] *= -1;
    }
    -1
}

#[cfg(test)]
mod tests {

    use super::first_duplicate_value;

    #[test]
    pub fn test_first_duplicate_value() {

        let mut arr = [2,1,3,4,5,6,2,7,8,9];

        assert_eq!(first_duplicate_value(&mut arr), 2);
    }
}