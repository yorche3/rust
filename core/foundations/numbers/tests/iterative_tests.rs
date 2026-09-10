use numbers::numbers;

#[test]
fn sum_of_first_n_ite() {
    assert_eq!(numbers::sum_of_first_n_ite(0), 0);
    assert_eq!(numbers::sum_of_first_n_ite(3), 6);
}

#[test]
fn factorial_ite() {
    assert_eq!(numbers::factorial_ite(0), 1);
    assert_eq!(numbers::factorial_ite(4), 24);
}

#[test]
fn fibonacci_ite() {
    assert_eq!(numbers::fibonacci_ite(0), 0);
    assert_eq!(numbers::fibonacci_ite(1), 1);
    assert_eq!(numbers::fibonacci_ite(6), 8);
}

#[test]
fn greatest_common_divisor_ite() {
    assert_eq!(numbers::greatest_common_divisor_ite(12, 8), 4);
    assert_eq!(numbers::greatest_common_divisor_ite(7, 5), 1);
}

#[test]
fn least_common_multiple_ite() {
    assert_eq!(numbers::least_common_multiple_ite(4, 6), 12);
    assert_eq!(numbers::least_common_multiple_ite(6, 8), 24);
}
