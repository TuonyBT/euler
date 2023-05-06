// Problem 1: sum of numbers less than n which are multiples of 3 or 5

pub fn multiples_of3and5(n: i32) -> i32 {
    multiples_of_x(3, n) + multiples_of_x(5, n) - multiples_of_x(15, n)
}

pub fn multiples_of_x(x: i32, n: i32) -> i32 {
    let limit = (n - 1) / x;
    x * limit * (limit + 1) / 2
}

// Problem 2: sum of even Fibonacci numbers up to n
pub fn fibo_even_sum(n: i32) -> i32 {
    let mut r = 0;
    let mut s = 2;
    let mut total = 0;
    while s <= n {
        let t = r + 4 * s;
        total += s;
        r = s;
        s = t;
    }

    let phi = (1.0 + 5f64.powf(0.5)) / 2.0;
    let k = ((phi * 5f64.powf(0.5) * n as f64 + 0.5).ln() / phi.ln()).floor();
    let total = ((phi.powf(k + 1.0 - (-1.0 / phi).powf(k + 1.0)) / (5f64.powf(0.5) - 1.0) / 2.0)).floor() as i32;

    total * 2
}