
// O(n ^ 3) time | O(n) space
pub fn three_sum(vec: Vec<i32>, target: i32) -> Vec<[i32; 3]> {
    let mut res: Vec<[i32; 3]> = vec![];

    for i in 0..vec.len() {
        for j in (i+1)..vec.len() {
            for k in (j+1)..vec.len() {
                if target == vec[i] + vec[j] + vec[k] {
                    res.push([vec[i], vec[j], vec[k]])
                }
            }
        }
    }

    return res
}

// O(n * log(n)) time | O(n) space
pub fn three_sum1(vec: Vec<i32>, target: i32) -> Vec<[i32; 3]> {
    let mut res: Vec<[i32; 3]> = Vec::new();

    let mut vec_copy = vec.clone(); // to avoid mutate passing vec
    vec_copy.sort();

    for i in 0..vec_copy.len() {
        if i > 0 && vec_copy[i - 1] == vec_copy[i] {
            continue;
        }

        let mut l = i + 1;
        let mut r = vec_copy.len() - 1;

        while l < r {
            let sum = vec_copy[i] + vec_copy[l] + vec_copy[r];
            if sum == target {
                res.push([vec_copy[i], vec_copy[l], vec_copy[r]]);
                l += 1;
                r -= 1;
                while l < r && vec_copy[l - 1] == vec_copy[l] {
                    l += 1;
                }
                while l < r && vec_copy[r] == vec_copy[r + 1] {
                    r -= 1;
                }
            } else if sum < target {
                l += 1;
            } else {
                r -= 1;
            }
        }
    }


    return res
}