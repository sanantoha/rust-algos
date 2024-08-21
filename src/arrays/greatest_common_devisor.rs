
// O(log(a + b)) time | O(1) space
pub fn gcd(mut a: i32, mut b: i32) -> i32 {

    while b != 0 {
        let tmp = a % b;
        a = b;
        b = tmp;
    }
    
    a
}

#[cfg(test)]
mod tests {
    use super::gcd;


    #[test]
    fn test_gcd() {
        let res = gcd(18, 6);
        println!("{res}");
        assert_eq!(res, 6);
    }

    #[test]
    fn test_gcd1() {
        let res = gcd(18, 10);
        println!("{res}");
        assert_eq!(res, 2);
    }

    #[test]
    fn test_gcd2() {
        let res = gcd(17, 11);
        println!("{res}");
        assert_eq!(res, 1);
    }

    #[test]
    fn test_gcd3() {
        let res = gcd(5, 15);
        println!("{res}");
        assert_eq!(res, 5);
    }
}