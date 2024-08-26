
// O(n) time | O(1) space
pub fn reverse_string(arr: &mut [char]) {
    let mut l = 0;
    let mut r = arr.len() - 1;

    while l < r {
        let tmp = arr[l];
        arr[l] = arr[r];
        arr[r] = tmp;
        l += 1;
        r -= 1;
    }
}

#[cfg(test)]
mod tests {

    use super::reverse_string;

    #[test]
    fn it_reverse_string() {

        let mut arr = ['h', 'e', 't', 'l', 'l', 'o'];

        reverse_string(&mut arr);

        assert_eq!(arr, ['o', 'l', 'l', 't', 'e', 'h']);
    }
}