

// O(n ^ 2) time | O(1) space
pub fn insert_sort(arr: &mut [i32]) {

    for i in 1..arr.len() {
        let mut j = i;
        let val = arr[i];

        while j > 0 && arr[j - 1] > val {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = val;
    }    
}

#[cfg(test)]
mod tests {

    use rand::Rng;

    use super::insert_sort;

    #[test]
    fn it_insert_sort() {

        let mut arr = [1, 9, 0, 3, 5, 7, 6, 8, 4, 2];

        insert_sort(&mut arr);

        assert_eq!(arr, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn it_insert_sort1() {

        let mut arr = [0; 30];

        let mut rng = rand::rng();

        for v in &mut arr {
            *v = rng.random_range(1..101);
        }

        insert_sort(&mut arr);
        
        println!("{:?}", arr);

        for i in 1..arr.len() {
            assert!(arr[i - 1] <= arr[i]);
        }
    }
}