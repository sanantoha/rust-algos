
// O(w * h) time | O(w * h) space
pub fn zig_zag_traverse(matrix: &[&[i32]]) -> Vec<i32> {
    if matrix.is_empty() {
        return vec![];
    }

    let mut res = vec![];

    let mut is_going_down = true;

    let mut row = 0;
    let mut col = 0;

    let height = matrix.len() - 1;
    let width = matrix[0].len() - 1;

    while row <= height && col <= width {
        res.push(matrix[row][col]);

        if is_going_down {
            if row == height || col == 0 {
                is_going_down = false;
                if row == height {
                    col += 1;
                } else {
                    row += 1;
                }
            } else {
                row += 1;
                col -= 1;
            }
        } else {
            if row == 0 || col == width {
                is_going_down = true;
                if col == width {
                    row += 1;
                } else {
                    col += 1;
                }
            } else {
                row -= 1;
                col += 1;
            }
        }
    }
    
    res
}

#[cfg(test)]
mod tests {

    use super::zig_zag_traverse;

    #[test]
    fn it_zig_zag_traverse() {
        let matrix: &[&[i32]] = &[
            &[1, 3, 4, 10],
            &[2, 5, 9, 11],
            &[6, 8, 12, 15],
            &[7, 13, 14, 16]
        ];

        assert_eq!(zig_zag_traverse(matrix), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
    }
}