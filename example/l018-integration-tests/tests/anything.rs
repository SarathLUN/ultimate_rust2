use l018_integration_tests::snuggle;

#[test]
fn it_work_from_outside() {
    assert!(snuggle(4) == 32);
}
