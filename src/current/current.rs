
pub fn disk_stacking(disks: &[(i32, i32, i32)]) -> Vec<(i32, i32, i32)> {
    vec![]
}

#[cfg(test)]
mod tests {
    use super::disk_stacking;

    #[test]
    fn test_disk_stacking() {
        let disks: &[(i32, i32, i32)] = &[
            (2, 1, 2),
            (3, 2, 3),
            (2, 2, 8),
            (2, 3, 4),
            (2, 2, 1),
            (4, 4, 5),
        ];

        let exp: Vec<(i32, i32, i32)> = vec![
            (2, 1, 2),
            (3, 2, 3),
            (4, 4, 5)
        ];

        let res = disk_stacking(disks);
        println!("{:?}", res);
        assert_eq!(res, exp);
    }
}