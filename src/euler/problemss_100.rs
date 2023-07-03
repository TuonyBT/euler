use std::collections::{BTreeMap, BTreeSet, HashSet};
use itertools::Itertools;
use rand::Rng;

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

pub fn sieve_of_eratothsenes_all(x: usize) -> Vec<usize> {
    let mut sieve = vec![true; x + 1];
    sieve[0] = false;
    sieve[1] = false;
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

//  This sieve tests only odd numbers
pub fn sieve_of_eratothsenes(x: usize) -> Vec<usize> {
    let mut sieve = vec![true; (x + 1) / 2];
    sieve[0] = false;

    let mut lp: usize = 3;
    while lp <= (x as f64).sqrt().floor() as usize {
        let fnp = lp.pow(2);
        for idx in (fnp..x + 1).step_by(lp * 2) {
            sieve[(idx - 1) / 2] = false;
        }
        lp = match sieve[(lp - 1) / 2 + 1..].iter().position(|z| z == &true) {
            Some(y) => (y + (lp - 1) / 2 + 1) * 2 + 1,
            None => x,
        };
    }
    let mut primes = sieve.iter().enumerate().filter(|z| z.1 == &true).map(|z| z.0 * 2 + 1).collect::<Vec<usize>>();
    primes.insert(0, 2);
    primes

}

// Problem 4: largest palindromic number which is the product of two 3-digit numbers

pub fn largest_palindromic(n: usize) -> usize {

    // The product of 2 n-digit numbers has 2(n-1) + x digits, where x is 1 or 2
    // Therefore the palindrome must have two ends of length n-1 and either one middle digit or two identical middle digits 
    let out_len = n - 1;

    // build a palindromic vector of digits that has the appropriate length
    for p in palin_ints(out_len as usize) {

    // convert this vector into an integer and find its square root
        let ords = p.iter().enumerate().map(|(idx, &z)| z * 10_i32.pow(p.len() as u32 - idx as u32 - 1) ).sum::<i32>();
        let st = (ords as f64).powf(0.5) as i32 + 1;

    // test all possible factors from sqrt down: at least one factor must be less than sqrt
        for ndf in (1..st).rev() {

    // is the cofactor an integer of the correct number of digits?
            if ords % ndf == 0 && ords / ndf < 10_i32.pow(n as u32) && ords / ndf >= 10_i32.pow(out_len as u32) {
                return ords as usize;
            }
        }
    }
    0
}

// build a palindromic vector of digits that has a given stem length
pub fn palin_ints(stem_length: usize) -> Vec::<Vec<i32>> {

    let mut palindromes = Vec::<Vec<i32>>::new();

    // even and odd lengths allow for one or two identical middle digits
    for even in [true, false].iter() {
        for stem in vec![(0..10).rev(); stem_length].into_iter().multi_cartesian_product() {
            if stem[0] == 0 {continue}
            for inner in (0i32..10).rev() {
                let mut p = stem.clone();
                p.push(inner);
                if even == &true {
                    p.push(inner);
                }
                p.extend(stem.iter().rev());
                palindromes.push(p);
            }
        }
    }
    palindromes
}

// Problem 5: smallest multiple of all numbers up to and including n

pub fn smallest_multiple(n: usize) -> usize {

    let u = (2..(n + 1)).collect::<BTreeSet<usize>>();
    let p = sieve_of_eratothsenes(n).into_iter().collect::<BTreeSet<usize>>();
    let c = u.difference(&p).collect::<BTreeSet<&usize>>();

    let mut prime_coeffs = p.iter().map(|&z| (z, 1usize)).collect::<BTreeMap<usize, usize>>();
    println!("Compound numbers in range of interest: {:?}", c);


    for cp in c.into_iter() {
        let mut prime_factors = BTreeMap::<usize, usize>::new();
        let mut m = *cp;
        for fact in p.iter() {
            while m / fact * fact == m {
                m /= fact;
                *prime_factors.entry(*fact).or_insert(0) += 1; 
            }
        }
        println!("Compound number {} has prime factors: {:?}", cp, prime_factors);
        for (k, v) in prime_factors.into_iter() {
            let new_coef = v.max(prime_coeffs[&k]);
            prime_coeffs.insert(k, new_coef); 
        }
        println!("After compound number {} coeffs of prime factors: {:?}", cp, prime_coeffs);
    }

    let result = prime_coeffs.into_iter().map(|(pr, cf)| (pr as u32).pow(cf as u32)).product::<u32>();

    result as usize

}

// Problem 6: sum square difference

pub fn sum_square_difference(n: u32) -> u32 {
// used respective formulae for sum(i) and sum(i_squared), then simplified expression for difference     
    (n.pow(2) - 1) * (3 * n + 2) * n / 12
}

//  Problem 7: nth_prime

pub fn nth_prime(n: usize) -> [usize; 2] {

    let up = nth_prime_bounds(n)[1];
    [sieve_of_eratothsenes(up)[n - 1], wheel_sieve(up)[n - 1]]

} 

//  Reverse wheel prime finder
pub fn wheel(n: usize) -> bool {

    let i = n / 30;
    let j = n % 30 / 6;
    let k = (n % 30) % 6;

    (k == 1 || k == 5 || [2, 3, 5].contains(&n) ) && n > 1 
}

//  Wheel with sieve applied
pub fn wheel_sieve(n: usize) -> Vec<usize> {

    let mut primes = vec![2, 3, 5];
    let spokes = [7, 11, 13, 17, 19, 23, 29, 31];

    for i in 0..n / 30 + 1 {
        for spoke in spokes {
            primes.push(i * 30 + spoke);
        }
    }

    let tests = primes.clone();
    for test in tests {
        if test * test > n {break}
        primes = primes.into_iter().filter(|z| z / test == 1 || z % test != 0).collect::<Vec<usize>>();
    }

    primes

} 

// Bounds for nth prime
pub fn nth_prime_bounds(n: usize) -> [usize; 2] {
    
    if n < 10 {return [30, 31]}
    else {
    
        let ln_n = (n as f64).ln();
        let ln_ln_n = ln_n.ln();

        return [((ln_n + ln_ln_n - 1.0) * n as f64).floor() as usize, ((ln_n + ln_ln_n) * n as f64).ceil() as usize]
    }
} 



//  Test for probable prime using Fermat primality test
pub fn fermat_prime(m: i64, k: i32) -> bool {
    let mut prob_prime = true;
    let mut rng = rand::thread_rng();

    for _k in 0..k {
        let mut rems = Vec::<i64>::new();
        let b: i64 = 2i64 + (rng.gen::<f64>() * (m - 4) as f64 / 1.0) as i64;
        let mut c: i64 = 1;
    
        for _e in 0..m - 1 {
            c = (b * c) % m;
            if rems.contains(&c) {break}
            else {rems.push(c);}
        }    
        let period = rems.len() as i64;
        if  (m - 1) % period != 0 {
                prob_prime = false;
        }
    }
    prob_prime
}