use calculator::calculator;

#[test]
fn addition() {
    assert_eq!(calculator::addition(2, 3), 5);
}

#[test]
fn subtraction() {
    assert_eq!(calculator::subtraction(5, 2), 3);
}

#[test]
fn multiplication() {
    assert_eq!(calculator::multiplication(3, 4), 12);
}

#[test]
fn division() {
    assert_eq!(calculator::division(10, 3), 3);
}

#[test]
fn modulus() {
    assert_eq!(calculator::modulus(10, 3), 1);
}
