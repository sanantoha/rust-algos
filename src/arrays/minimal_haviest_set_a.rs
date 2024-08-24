
// O(n * log(n)) time | O(1) space
pub fn minimal_heaviest_set_a(arr: &[i32]) -> Vec<i32> {
    let mut vec = arr.to_vec();
    vec.sort();

    let mut l = 0;
    let mut r = vec.len() - 1;
    let mut left_sum = vec[l];
    let mut right_sum = vec[r];

    while l < r {
        if left_sum > right_sum {
            r -= 1;
            right_sum += vec[r];
        } else {
            l += 1;
            left_sum += vec[l];
        }
    }    

    vec[l..].to_vec()
}

#[cfg(test)]
mod tests {

    use super::minimal_heaviest_set_a;

    #[test]
    fn it_minimal_heaviest_set_a() {

        let arr = &[6, 4, 2, 3, 1, 5];

        assert_eq!(minimal_heaviest_set_a(arr), vec![5, 6]);
    }
}