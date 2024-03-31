fn main() {
    println!("starting the test:");
}

#[cfg(test)]
mod test {
    #[test]
    fn the_bin_test() {
        assert_eq!(1 + 1, 2);
    }
}
