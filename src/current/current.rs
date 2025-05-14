
pub fn merge_sort(arr: &[i32]) -> Vec<i32> {
    vec![]
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

        let mut rng = rand::thread_rng();

        for v in &mut arr {
            *v = rng.gen_range(1..101);
        }

        let res = merge_sort(&arr);

        println!("{:?}", res);

        for i in 1..arr.len() {
            let s = format!("{} > {}", res[i - 1], res[i]);
            assert!(res[i - 1] <= res[i], "{}", &s);
        }
    }
}