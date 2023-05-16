use itertools::Itertools;
use std::collections::{BTreeMap, BTreeSet};

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

// Problem 3: largest prime factor

pub fn largest_prime_factor(n: usize) -> usize {
    let mut prime_factors = BTreeMap::<usize, usize>::new();
    let mut m = n;


//  Try reducing the size of the primes vector we need to test by pulling out first few primes manually
    for seed in [2, 3, 5].into_iter() {
        while m / seed * seed == m {
            m /= seed;
            *prime_factors.entry(seed).or_insert(0) += 1; 
        }
    }


//  if the cofactor of the seeds we have tested is less than the square of the next prime, the slice indexation will not work...
    if m > 48 {
        let lim = (m as f64).powf(0.5) as usize + 1;
        for p in &sieve_of_eratothsenes(lim)[5..] {
            let seed = *p as usize;
            while m / seed * seed == m {
                m /= seed;
                *prime_factors.entry(seed).or_insert(0) += 1; 
            }
        }
    }

//  in which case the cofactor itself must be prime
    else if m > 1 {
        prime_factors.insert(m, 1);         
    }

    let hpf = match prime_factors.keys().last() {
        Some(z) => *z,
        None => 1,
    };
    hpf
}

pub fn sieve_of_eratothsenes(x: usize) -> Vec<usize> {
    let mut sieve = vec![true; x + 1];
    let mut lp: usize = 2;
    while lp <= (x as f64).sqrt().floor() as usize {
        let fnp = lp.pow(2);
        for idx in (fnp..sieve.len()).step_by(lp) {
            sieve[idx] = false;
        }
        lp = match sieve[lp + 1..].iter().position(|z| z == &true) {
            Some(y) => y + lp + 1,
            None => x,
        };
    }
    let primes = sieve.iter().enumerate().filter(|z| z.1 == &true).map(|z| z.0).collect::<Vec<usize>>();
    primes

}

// Problem 4: largest palindromic number which is the product of two 3-digit numbers

pub fn largest_palindromic(n: usize) -> usize {

    let out_len = n - 1;
    let ndfs = n_digit_factors(n as i32);
    let result = 0_usize;


    for p in palin_ints(out_len as usize) {
        let ords = p.iter().enumerate().map(|(idx, &z)| z * 10_i32.pow(p.len() as u32 - idx as u32 - 1) ).sum::<i32>();

        for ndf in &ndfs {
            if ords % ndf == 0 && ndfs.contains(&(ords / ndf))  {
                println!("factors {} {}", ndf, ords / ndf);
                return ords as usize

            }
        }
    }
    result
}

fn palin_ints(stem_length: usize) -> Vec::<Vec<i32>> {

    let mut palindromes = Vec::<Vec<i32>>::new();

    for even in [true, false].iter() {
        for stem in vec![(0..10).rev(); stem_length].into_iter().multi_cartesian_product() {
            if stem[0] == 0 {continue}
            for inner in (0i32..10).rev() {
                let mut p = stem.clone();
                p.push(inner);
                if even == &true {
                    p.push(inner);
                }
                for e in stem.iter().rev() {
                    p.push(*e);
                }
                palindromes.push(p);
            }
        }
    }

    palindromes
}

pub fn n_digit_factors(n: i32) -> Vec<i32> {
    let mut facs = BTreeSet::<i32>::new(); 
    let mut mults = BTreeSet::<i32>::new(); 

    let low = 10i32.pow(n as u32 -1);
    let hi = low * 9;
    for i in 0..hi {
        facs.insert(low + i);
    }

    for j in 2..10 {
        for i in 0..low {
            let lij = (low + i) * j;
            if lij >= low * 10 {break}
            for m in [1, 2, 3, 5, 7] {
                if lij * m >= low * 10 {break}
                mults.insert(lij * m);
            }
        }
    }
    facs.difference(&mults).map(|&z| z).collect::<Vec<i32>>()
}

