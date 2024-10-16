use rand::Rng;
use rand::rngs::ThreadRng;

// O(n * log(n)) time | O(log(n)) space
pub fn quick_sort(arr: &mut [i32]) {
    let mut rand = rand::thread_rng();
    sort(arr, 0, arr.len() - 1, &mut rand);
}

fn sort(arr: &mut [i32], l: usize, r: usize, rand: &mut ThreadRng) {
    if l > r {
        return;
    }

    let p = partition(arr, l, r, rand);

    if p > 0 {
        sort(arr, l, p - 1, rand);
    }
    sort(arr, p + 1, r, rand);
}

fn partition(arr: &mut [i32], l: usize, r: usize, rand: &mut ThreadRng) -> usize {
    let p = rand.gen_range(l..=r);

    let mut j = l;
    arr.swap(r, p);

    for i in l..r {
        if arr[i] <= arr[r] {
            arr.swap(i, j);
            j += 1;
        }
    }

    arr.swap(j, r);
    j
}


#[cfg(test)]
mod tests {
    use rand::thread_rng;
    use super::*;

    #[test]
    fn test_quick_sort() {
        let mut arr = vec![5, 2, 4, 6, 1, 3];

        quick_sort(&mut arr);

        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_quick_sort_case1() {
        let mut arr = vec![0; 30];

        let mut rand = thread_rng();

        for i in 0..arr.len() {
            arr[i] = rand.gen_range(0..100)
        }

        quick_sort(&mut arr);

        println!("{:?}", arr);

        for i in 1..arr.len() {
            assert!(arr[i - 1] <= arr[i]);
        }
    }
}