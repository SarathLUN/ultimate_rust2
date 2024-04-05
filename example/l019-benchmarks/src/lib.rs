/// # Example
///
/// ```
/// # use l018_integration_tests::snuggle;
/// let bunnies = snuggle(5);
/// assert_eq!(bunnies, 40);
/// ```
// Benmark#3: bit shifting approach
pub fn snuggle(bunnies: u128) -> u128 {
    bunnies << 3
}

// Benmark#1: the typical, multiplication approach
// pub fn snuggle(bunnies: u128) -> u128 {
//      bunnies * 8
// }

// Benmark#2: the loop approach
// pub fn snuggle(bunnies: u128) -> u128 {
//      let mut result = 0;
//      for _ in 0..8 {
//          result += bunnies
//      }
//      result
// }

#[cfg(test)]
mod test {
    use super::*;
    use std::num::ParseIntError;

    #[test]
    fn snuggle_bunnies_multiply() {
        assert_eq!(snuggle(2), 16);
    }

    // #[should_panic]
    // #[test]
    // fn scared_bunny() {
    //     panic!("Hop hopping hop!");
    // }

    #[test]
    fn bunny_result() -> Result<(), ParseIntError> {
        let num_bunnies: u64 = "4".parse()?;
        assert_eq!(num_bunnies, 4);
        Ok(())
    }
}
