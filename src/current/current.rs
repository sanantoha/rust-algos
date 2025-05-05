
pub fn stable_internships(interns: &[&[i32]], teams: &[&[i32]]) -> Vec<(usize, usize)> {
    vec![]
}


#[cfg(test)]
mod tests {
    use crate::arrays::stable_internships::stable_internships;
    use super::stable_internships;

    #[test]
    fn it_stable_internships() {

        let interns: &[&[usize]] = &[
            &[0, 1, 2],
            &[0, 2, 1],
            &[1, 2, 0],
        ];

        let teams: &[&[usize]] = &[
            &[2, 1, 0],
            &[0, 1, 2],
            &[0, 1, 2]
        ];

        let mut res = stable_internships(interns, teams);
        res.sort_by_key(|x| x.0);

        println!("{:?}", res);

        assert_eq!(res, vec![(0, 1), (1, 0), (2, 2)])
    }
}