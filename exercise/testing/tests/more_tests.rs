use testing::{splish, sploosh};

#[test]
fn sploosh_integrate_test() {
    assert_eq!(sploosh(splish(-1, 0), splish(1, 1), splish(3, 2)), 4);
}
