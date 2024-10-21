
// O(1) time | O(1) space - because length of valid string is short not more than 12 (ip pattern: xxx.xxx.xxx.xxx)
pub fn valid_ip_addresses(str: &str) -> Vec<String> {
    if str.is_empty() {
        return vec![];
    }

    let mut address: Vec<String> = vec![String::default(); 4];

    let mut res = vec![];

    for i in 1..(str.len().min(4)) {
        address[0] = str[..i].to_owned();

        if !is_valid(&address[0]) {
            continue;
        }

        for j in i..(str.len().min(i + 4)) {
            address[1] = str[i..j].to_owned();

            if !is_valid(&address[1]) {
                continue;
            }

            for k in j..(str.len().min(j + 4)) {
                address[2] = str[j..k].to_owned();
                address[3] = str[k..].to_owned();

                if is_valid(&address[2]) && is_valid(&address[3]) {
                    res.push(address.join("."))
                }
            }
        }
    }

    res
}

fn is_valid(s: &str) -> bool {
    if let Ok(i) = s.parse::<u8>() {
        return i.to_string() == s
    }
    false
}

#[cfg(test)]
mod tests {
    use crate::strings::valid_ip_addresses::valid_ip_addresses;

    #[test]
    fn test_valid_ip_addresses() {
        let s = String::from("1921680");

        let res = valid_ip_addresses(&s);
        println!("{:?}", res);
        assert_eq!(res, ["1.9.216.80", "1.92.16.80", "1.92.168.0", "19.2.16.80", "19.2.168.0", "19.21.6.80", "19.21.68.0", "19.216.8.0", "192.1.6.80", "192.1.68.0", "192.16.8.0"]);
    }
}