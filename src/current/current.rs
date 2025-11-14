

pub fn minimal_heaviest_set_a(_arr: &[i32]) -> Vec<i32> {
    vec![]
}

#[cfg(test)]
mod tests {

    use super::minimal_heaviest_set_a;

    #[test]
    fn it_minimal_heaviest_set_a() {

        let arr = &[6, 4, 2, 3, 1, 5];

        assert_eq!(minimal_heaviest_set_a(arr), vec![5, 6]);
    }
}