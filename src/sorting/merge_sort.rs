
// O(n * log(n)) time | O(n) space
pub fn merge_sort(arr: &[i32]) -> Vec<i32> {
    let mut result = arr.to_vec();
    let mut origin = arr.to_vec();
    sort(&mut origin, 0, arr.len() - 1, &mut result);
    result
}

fn sort(arr: &mut [i32], l: usize, r: usize, result: &mut [i32]) {
    if l >= r {
        return;
    }

    let mid = l + (r - l) / 2;
    sort(result, l, mid, arr);
    sort(result, mid + 1, r, arr);
    merge(arr, l, mid, r, result);
}

fn merge(arr: &[i32], l: usize, mid: usize, r: usize, result: &mut [i32]) {
    let mut i = l;
    let mut j= mid + 1;
    let mut idx = l;
    while i <= mid && j <= r {
        if arr[i] <= arr[j] {
            result[idx] = arr[i];
            i += 1;
            idx += 1;
        } else {
            result[idx] = arr[j];
            j += 1;
            idx += 1;
        }
    }

    while i <= mid {
        result[idx] = arr[i];
        i += 1;
        idx += 1;
    }

    while j <= r {
        result[idx] = arr[j];
        j += 1;
        idx += 1;
    }
}

#[cfg(test)]
mod tests {

    use super::merge_sort;
    use rand::Rng;

    #[test]
    fn it_merge_sort() {
        let res = merge_sort(&[8, 3, 6, 8, 3, 1, 5, 7, 8, 9]);
        println!("{res:?}");
        assert_eq!(res, vec![1, 3, 3, 5, 6, 7, 8, 8, 8, 9]);
    }

    #[test]
    fn it_merge_sort1() {
        let mut arr = [0; 30];

        let mut rng = rand::rng();

        for v in &mut arr {
            *v = rng.random_range(1..101);
        }

        let res = merge_sort(&arr);
        
        println!("{:?}", res);

        for i in 1..arr.len() {
            let s = format!("{} > {}", res[i - 1], res[i]);
            assert!(res[i - 1] <= res[i], "{}", &s);
        }
    }
}