// Direct recursion (_rec)

pub fn sum_of_first_n_rec(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    n + sum_of_first_n_rec(n - 1)
}

pub fn factorial_rec(n: i32) -> i32 {
    if n == 0 {
        return 1;
    }
    n * factorial_rec(n - 1)
}

pub fn fibonacci_rec(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    fibonacci_rec(n - 1) + fibonacci_rec(n - 2)
}

pub fn greatest_common_divisor_rec(a: i32, b: i32) -> i32 {
    if b == 0 {
        return a;
    }
    greatest_common_divisor_rec(b, a % b)
}

pub fn least_common_multiple_rec(a: i32, b: i32) -> i32 {
    (a * b) / greatest_common_divisor_rec(a, b)
}

// Accumulator recursion (_acc): educational bridge, no guaranteed TCO in Rust

pub fn sum_of_first_n_acc(n: i32) -> i32 {
    sum_of_first_n_acc_help(n, 0)
}

fn sum_of_first_n_acc_help(n: i32, acc: i32) -> i32 {
    if n <= 0 {
        return acc;
    }
    sum_of_first_n_acc_help(n - 1, n + acc)
}

pub fn factorial_acc(n: i32) -> i32 {
    factorial_acc_help(n, 1)
}

fn factorial_acc_help(n: i32, acc: i32) -> i32 {
    if n <= 1 {
        return acc;
    }
    factorial_acc_help(n - 1, n * acc)
}

pub fn fibonacci_acc(n: i32) -> i32 {
    fibonacci_acc_help(n, 0, 1)
}

fn fibonacci_acc_help(n: i32, acc2: i32, acc1: i32) -> i32 {
    if n <= 0 {
        return acc2;
    }
    if n <= 2 {
        return acc1 + acc2;
    }
    fibonacci_acc_help(n - 1, acc1, acc1 + acc2)
}

pub fn greatest_common_divisor_acc(a: i32, b: i32) -> i32 {
    greatest_common_divisor_acc_help(a, b)
}

fn greatest_common_divisor_acc_help(a: i32, b: i32) -> i32 {
    if b == 0 {
        return a;
    }
    greatest_common_divisor_acc_help(b, a % b)
}

pub fn least_common_multiple_acc(a: i32, b: i32) -> i32 {
    (a * b) / greatest_common_divisor_acc(a, b)
}

// Iterative (_ite)

pub fn sum_of_first_n_ite(n: i32) -> i32 {
    let mut result = 0;
    for i in 1..=n {
        result += i;
    }
    result
}

pub fn factorial_ite(n: i32) -> i32 {
    let mut result = 1;
    for i in 2..=n {
        result *= i;
    }
    result
}

pub fn fibonacci_ite(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    let mut acc2 = 0;
    let mut acc1 = 1;
    for _ in 2..=n {
        let temp = acc1 + acc2;
        acc2 = acc1;
        acc1 = temp;
    }
    acc1
}

pub fn greatest_common_divisor_ite(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

pub fn least_common_multiple_ite(a: i32, b: i32) -> i32 {
    (a * b) / greatest_common_divisor_ite(a, b)
}
