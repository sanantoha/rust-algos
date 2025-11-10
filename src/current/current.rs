
pub fn river_sizes(_matrix: &[&[i32]]) -> Vec<i32> {
    vec![]
}


#[cfg(test)]
mod tests {

    use super::river_sizes;

    #[test]
    pub fn test_river_sizes() {

        let matrix: &[&[i32]] = &[
            &[1, 0, 0, 1, 0],
            &[1, 0, 1, 0, 1],
            &[0, 0, 1, 0, 1],
            &[1, 0, 1, 1, 0],
            &[1, 0, 1, 0, 0],
        ];

        let res = river_sizes(matrix);
        println!("{:?}", res);

        assert_eq!(res, vec![2, 1, 5, 2, 2])
    }
}