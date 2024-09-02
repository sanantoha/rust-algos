
// O(w * h) time | O(w * h) space
pub fn spiral(matrix: &[&[i32]]) -> Vec<i32> {
    if matrix.is_empty() {
        return vec![];
    }

    let mut res = vec![];

    let mut start_row = 0;
    let mut end_row = matrix.len() - 1;
    let mut start_col = 0;
    let mut end_col = matrix[start_row].len() - 1;

    while start_row <= end_row && start_col <= end_col {
        for i in start_col..=end_col {
            res.push(matrix[start_row][i]);
        }
        start_row += 1;

        for i in start_row..=end_row {
            res.push(matrix[i][end_col]);
        }
        end_col -= 1;

        if start_row <= end_row && start_col <= end_col {
            for i in (start_col..=end_col).rev() {
                res.push(matrix[end_row][i]);
            }
            end_row -= 1;

            for i in (start_row..=end_row).rev() {
                res.push(matrix[i][start_col]);
            }
            start_col += 1;
        }
    }

    res
}

#[cfg(test)]
mod tests {

    use super::spiral;

    #[test]
    fn it_spiral() {
        let matrix: &[&[i32]] = &[
            &[ 1,  2,  3,  5,  6,  7],
            &[19, 20, 21, 22, 23,  8],
            &[18, 29, 30, 31, 24,  9],
            &[17, 28, 27, 26, 25, 10],
            &[16, 15, 14, 13, 12, 11],
        ];

        assert_eq!(spiral(matrix), vec![1, 2, 3, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 
            16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31]);
    }
}