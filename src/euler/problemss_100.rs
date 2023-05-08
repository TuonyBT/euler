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

//  recursive approach adding each term in turn
//  only uses integers

//    let mut r = 0;
//    let mut s = 2;
//    let mut total = 0;
//    while s <= n {
//        let t = r + 4 * s;
//        total += s;
//        r = s;
//        s = t;
//    }
//    total


//  one-shot approach using Binet's formula and its corollary
//  requires conversion into floats, some fp maths including logs and powers, then conversion back to ints
    
//  index of the nearest even Fibonacci number less than or equal to n, plus two    
    let k = fibo_index(n) / 3 * 3 + 2;
    (binet(k) - 1) / 2
}

//  Binet's formula for the value of the nth Fibonacci number
pub fn binet(n: i32) -> i32 {

    let root_five: f64 = 5f64.powf(0.5);
    let phi: f64 = (1.0 + root_five) / 2.0;

    ((phi.powf(n as f64) - (1.0 - phi).powf(n as f64)) / root_five) as i32
}

//  Corollary of Binet finds the index of the smallest Fibonacci number greater than or equal to n
pub fn fibo_index(n: i32) -> i32 {

    let root_five: f64 = 5f64.powf(0.5);
    let phi: f64 = (1.0 + root_five) / 2.0;

    ((n as f64 * root_five - 0.5).ln() / phi.ln()).ceil() as i32

}

