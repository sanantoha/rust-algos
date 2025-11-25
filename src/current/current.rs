

pub fn reverse_string(arr: &mut [char]) {
    
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
}