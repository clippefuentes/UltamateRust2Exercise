use testing::{splish, sploosh};

#[test]
fn test_more_test() {
  assert!(sploosh(splish(-1, 0), splish(1, 1), splish(3, 2)) == 4);
}