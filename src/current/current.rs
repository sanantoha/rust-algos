

pub fn phone_number_mnemonics(src: &str) -> Vec<String> {
    vec![]
}

#[cfg(test)]
mod tests {
    use super::phone_number_mnemonics;

    #[test]
    fn it_phone_number_mnemonics() {
        let res = phone_number_mnemonics("1905");
        println!("{:?}", res);

        assert_eq!(res, vec!["1w0j", "1w0k", "1w0l", "1x0j", "1x0k", "1x0l",
                             "1y0j", "1y0k", "1y0l", "1z0j", "1z0k", "1z0l"]);
    }
}