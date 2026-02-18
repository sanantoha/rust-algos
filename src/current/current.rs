pub fn sort_k_sorted_array(arr: &mut [i32], k: i32) -> Vec<i32> {
    vec![]
}

#[cfg(test)]
mod tests {
    use super::sort_k_sorted_array;

    #[test]
    fn basic() {
        let mut arr: Vec<i32> = vec![3, 2, 1, 5, 4, 7, 6, 5];

        assert_eq!(sort_k_sorted_array(&mut arr, 3), vec![1, 2, 3, 4, 5, 5, 6, 7]);
    }
}