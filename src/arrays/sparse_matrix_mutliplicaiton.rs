
// O(m * k * n) time | O(m * n) space
pub fn multiply(mat1: &[&[i32]], mat2: &[&[i32]]) -> Vec<Vec<i32>> {
    if mat1.is_empty() || mat2.is_empty() || mat1[0].len() != mat2.len() {
        return vec![];
    }
    let mut res = vec![vec![0; mat2[0].len()]; mat1.len()];

    for i in 0..mat1.len() {
        for j in 0..mat1[i].len() {
            for k in 0..mat2[j].len() {
                res[i][k] += mat1[i][j] * mat2[j][k];
            }
        }
    }
    
    res
}

// O(m * k * n) time | O(m * k + k * n) space
pub fn multiply1(mat1: &[&[i32]], mat2: &[&[i32]]) -> Vec<Vec<i32>> {
    if mat1.is_empty() || mat2.is_empty() || mat1[0].len() != mat2.len() {
        return vec![];
    }
    let mut res = vec![vec![0; mat2[0].len()]; mat1.len()];


    let m1 = compress(mat1);
    let m2 = compress(mat2);

    for i in 0..mat1.len() {
        for (j, v1) in &m1[i] {
            for (k, v2) in &m2[*j] {
                res[i][*k] += v1 * v2;
            }
        }
    }
    
    res
}

fn compress(matrix: &[&[i32]]) -> Vec<Vec<(usize, i32)>> {
    let mut res = vec![];
    for i in 0..matrix.len() {
        res.push(vec![]);
        for j in 0..matrix[i].len() {
            if matrix[i][j] != 0 {
                res[i].push((j, matrix[i][j]));
            }
        }
    }
    res
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