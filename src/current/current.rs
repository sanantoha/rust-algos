
pub fn zig_zag_traverse(matrix: &[&[i32]]) -> Vec<i32> {
    vec![]
}

#[cfg(test)]
mod tests {
    use crate::arrays::zig_zag_traverse::zig_zag_traverse;
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