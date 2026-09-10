use numbers::numbers;

#[test]
fn sum_of_first_n_rec() {
    assert_eq!(numbers::sum_of_first_n_rec(0), 0);
    assert_eq!(numbers::sum_of_first_n_rec(3), 6);
}

#[test]
fn factorial_rec() {
    assert_eq!(numbers::factorial_rec(0), 1);
    assert_eq!(numbers::factorial_rec(4), 24);
}

#[test]
fn fibonacci_rec() {
    assert_eq!(numbers::fibonacci_rec(0), 0);
    assert_eq!(numbers::fibonacci_rec(1), 1);
    assert_eq!(numbers::fibonacci_rec(6), 8);
}

#[test]
fn greatest_common_divisor_rec() {
    assert_eq!(numbers::greatest_common_divisor_rec(12, 8), 4);
    assert_eq!(numbers::greatest_common_divisor_rec(7, 5), 1);
}

#[test]
fn least_common_multiple_rec() {
    assert_eq!(numbers::least_common_multiple_rec(4, 6), 12);
    assert_eq!(numbers::least_common_multiple_rec(6, 8), 24);
}
