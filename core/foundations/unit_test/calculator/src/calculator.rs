pub fn addition(a: i32, b: i32) -> i32 {
    a + b
}

pub fn subtraction(a: i32, b: i32) -> i32 {
    a - b
}

pub fn multiplication(a: i32, b: i32) -> i32 {
    let mut result = 0;
    for _ in 0..b {
        result = addition(result, a);
    }
    result
}

pub fn division(mut a: i32, b: i32) -> i32 {
    let mut quotient = 0;
    while a >= b {
        a = subtraction(a, b);
        quotient = addition(quotient, 1);
    }
    quotient
}

pub fn modulus(a: i32, b: i32) -> i32 {
    let quotient = division(a, b);
    let product = multiplication(quotient, b);
    subtraction(a, product)
}
