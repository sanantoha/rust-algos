

pub fn rotate(arr: &mut Vec<Vec<i32>>) {

}


#[cfg(test)]
mod tests {

    use super::rotate;

    #[test]
    fn it_rotate() {

        /*
    1 2 3  =>  1 4 7  => 7 4 1
    4 5 6  =>  2 5 8  => 8 5 2
    7 8 9  =>  3 6 9  => 9 6 3
        */

        let mut matrix = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9]
        ];

        let exp = vec![
            vec![7, 4, 1],
            vec![8, 5, 2],
            vec![9, 6, 3]
        ];

        rotate(&mut matrix);

        println!("{:?}", matrix);
        assert_eq!(matrix, exp)
    }
}