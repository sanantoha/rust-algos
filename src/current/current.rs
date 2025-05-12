
pub fn first_duplicate_value(arr: &mut [i32]) -> i32 {
    -1
}

#[cfg(test)]
mod tests {
    use crate::arrays::first_duplicate_value::first_duplicate_value;
    use super::first_duplicate_value;

    #[test]
    pub fn test_first_duplicate_value() {

        let mut arr = [2,1,3,4,5,6,2,7,8,9];

        assert_eq!(first_duplicate_value(&mut arr), 2);
    }
}