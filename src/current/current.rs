

pub fn multiply(mat1: &[&[i32]], mat2: &[&[i32]]) -> Vec<Vec<i32>> {
    vec![]
}

pub fn multiply1(mat1: &[&[i32]], mat2: &[&[i32]]) -> Vec<Vec<i32>> {
    vec![]
}


#[cfg(test)]
mod tests {
    use super::multiply;
    use super::multiply1;

    const MAT1: &[&[i32]] = &[
        &[1, 0, 0],
        &[-1, 0, 3]
    ];

    const MAT2: &[&[i32]] = &[
        &[7, 0, 0],
        &[0, 0, 0],
        &[0, 0, 1]
    ];

    const EXP_RES: &[&[i32]] = &[
        &[7,0,0],
        &[-7,0,3]
    ];

    #[test]
    fn it_multiply() {

        assert_eq!(multiply(MAT1, MAT2), EXP_RES);
    }

    #[test]
    fn it_multiply1() {

        assert_eq!(multiply1(MAT1, MAT2), EXP_RES);
    }
}