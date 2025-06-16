
pub fn reverse_string(arr: &[char]) {

}

pub fn rob(arr: &[i32]) -> i32 {
    0
}

pub fn rob1(arr: &[i32]) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::reverse_string;

    #[test]
    fn it_reverse_string() {

        let mut arr = ['h', 'e', 't', 'l', 'l', 'o'];

        reverse_string(&mut arr);

        assert_eq!(arr, ['o', 'l', 'l', 't', 'e', 'h']);
    }

    use super::{rob, rob1};

    #[test]
    fn it_rob() {

        assert_eq!(rob(&[4,1,2,7,5,3,1]), 14);
    }

    #[test]
    fn it_rob1() {

        assert_eq!(rob1(&[4,1,2,7,5,3,1]), 14);
    }
}