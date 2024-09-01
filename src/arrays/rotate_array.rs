
// O(n) time | O(n) space
pub fn rotate(arr: &mut [i32], k: i32) {
    let copy = arr.to_owned();

    for i in 0..arr.len() {
        let idx = (i + k as usize) % arr.len();
        arr[idx] = copy[i];
    }
}

// O(n) time | O(1) space
pub fn rotate1(arr: &mut [i32], k: i32) {
    if k <= 0 {
        return;
    }
    let n = (k as usize) % arr.len();
    reverse(arr, 0, arr.len() - 1);
    reverse(arr, 0, n - 1);
    reverse(arr, n, arr.len() - 1);
}

fn reverse(arr: &mut [i32], mut l: usize, mut r: usize) {
    while l < r {
        arr.swap(l, r);
        l += 1;
        r -= 1;
    }
}

#[cfg(test)]
mod tests {

    use super::rotate;
    use super::rotate1;

    #[test]
    fn it_rotate_arr() {
        let mut arr = [1, 2, 3, 4, 5, 6, 7];

        rotate(&mut arr, 3);

        println!("{arr:?}");
        assert_eq!(arr, [5, 6, 7, 1, 2, 3, 4])
    }

    #[test]
    fn it_rotate_arr1() {
        let mut arr = [-1,-100,3,99];

        rotate(&mut arr, 2);

        println!("{arr:?}");
        assert_eq!(arr, [3, 99, -1, -100])
    }

    #[test]
    fn it_rotate_arr2() {
        let mut arr = [1, 2, 3];

        rotate(&mut arr, 2);

        println!("{arr:?}");
        assert_eq!(arr, [2, 3, 1])
    }

    #[test]
    fn it_rotate1_arr() {
        let mut arr = [1, 2, 3, 4, 5, 6, 7];

        rotate1(&mut arr, 3);

        println!("{arr:?}");
        assert_eq!(arr, [5, 6, 7, 1, 2, 3, 4])
    }

    #[test]
    fn it_rotate1_arr1() {
        let mut arr = [-1,-100,3,99];

        rotate1(&mut arr, 2);

        println!("{arr:?}");
        assert_eq!(arr, [3, 99, -1, -100])
    }

    #[test]
    fn it_rotate1_arr2() {
        let mut arr = [1, 2, 3];

        rotate1(&mut arr, 2);

        println!("{arr:?}");
        assert_eq!(arr, [2, 3, 1])
    }

    
}