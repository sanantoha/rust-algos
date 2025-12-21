
pub fn rotate(arr: &mut [i32], k: i32) {
    
}

pub fn rotate1(arr: &mut [i32], k: i32) {
    
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