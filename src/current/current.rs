
pub fn valid_starting_city(distances: &[i32], fuel: &[i32], mpg: i32) -> Option<usize> {
    None
}

pub fn valid_starting_city1(distances: &[i32], fuel: &[i32], mpg: i32) -> Option<usize> {
    None
}

#[cfg(test)]
mod tests {
    use super::valid_starting_city;
    use super::valid_starting_city1;

    const DISTANCES: &[i32] = &[5, 25, 15, 10, 15];
    const FUEL: &[i32] = &[1, 2, 1, 0, 3];
    const MPG: i32 = 10;
    const EXP: usize = 4;

    #[test]
    fn it_valid_starting_city() {
        assert_eq!(valid_starting_city(DISTANCES, FUEL, MPG), Some(EXP));
    }

    #[test]
    fn it_valid_starting_city1() {
        assert_eq!(valid_starting_city1(DISTANCES, FUEL, MPG), Some(EXP));
    }
}