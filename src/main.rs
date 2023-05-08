pub mod euler;
pub use crate::euler::problemss_100::*;

fn main() {
    for n in 1..50 {
        println!("{} {}", n, largest_prime_factor(n));
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem_1() {
        assert_eq!(multiples_of3and5(10), 23);
        assert_eq!(multiples_of3and5(49), 543);
        assert_eq!(multiples_of3and5(1000), 233168);
        assert_eq!(multiples_of3and5(8456), 16687353);
        assert_eq!(multiples_of3and5(19564), 89301183);
    }
    #[test]
    fn problem_2() {
        assert_eq!(fibo_even_sum(1), 0);
        assert_eq!(fibo_even_sum(2), 2);
        assert_eq!(fibo_even_sum(8), 10);
        assert_eq!(fibo_even_sum(10), 10);
        assert_eq!(fibo_even_sum(34), 44);
        assert_eq!(fibo_even_sum(60), 44);
        assert_eq!(fibo_even_sum(1000), 798);
        assert_eq!(fibo_even_sum(100000), 60696);
        assert_eq!(fibo_even_sum(4000000), 4613732);
    }
    #[test]
    fn problem_3() {
        assert_eq!(largest_prime_factor(2), 2);
        assert_eq!(largest_prime_factor(3), 3);
        assert_eq!(largest_prime_factor(5), 5);
        assert_eq!(largest_prime_factor(7), 7);
        assert_eq!(largest_prime_factor(8), 2);
        assert_eq!(largest_prime_factor(13195), 29);
        assert_eq!(largest_prime_factor(600851475143), 6857);
    }
}

