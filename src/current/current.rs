

pub fn valid_ip_addresses(str: &str) -> Vec<String> {
    vec![]
}

#[cfg(test)]
mod tests {
    use super::valid_ip_addresses;

    #[test]
    fn test_valid_ip_addresses() {
        let s = String::from("1921680");

        let res = valid_ip_addresses(&s);
        println!("{:?}", res);
        assert_eq!(res, ["1.9.216.80", "1.92.16.80", "1.92.168.0", "19.2.16.80", "19.2.168.0", "19.21.6.80", "19.21.68.0", "19.216.8.0", "192.1.6.80", "192.1.68.0", "192.16.8.0"]);
    }
}