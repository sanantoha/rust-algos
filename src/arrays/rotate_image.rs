
// O(w * h) time | O(1) space
pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    transpose(matrix);
    reflect(matrix);
}

fn transpose(matrix: &mut Vec<Vec<i32>>) {

    for i in 0..matrix.len() {
        for j in (i + 1)..matrix.len() {
            let tmp = matrix[i][j];
            matrix[i][j] = matrix[j][i];
            matrix[j][i] = tmp;
        }
    }
}

fn reflect(matrix: &mut Vec<Vec<i32>>) {
    for arr in matrix {
        let mut l = 0;
        let mut r = arr.len() - 1;
        while l < r {
            arr.swap(l, r);
            l += 1;
            r -= 1;
        }
    }
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