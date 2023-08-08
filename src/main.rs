pub mod euler;
pub use crate::euler::problemss_100::*;

fn main() {

    for n in 1000..1001 {

            println!("n: {} Pythagorean triangle {:?}", n, pythag_triad_sum(n));
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

    #[test]
    fn problem_4() {
        assert_eq!(largest_palindromic(2), 9009);
        assert_eq!(largest_palindromic(3), 906609);
    }

    #[test]
    fn problem_5() {
        assert_eq!(smallest_multiple(5), 60);
        assert_eq!(smallest_multiple(7), 420);
        assert_eq!(smallest_multiple(10), 2520);
        assert_eq!(smallest_multiple(13), 360360);
        assert_eq!(smallest_multiple(20), 232792560);
    }

    #[test]
    fn problem_6() {
        assert_eq!(sum_square_difference(10), 2640);
        assert_eq!(sum_square_difference(20), 41230);
        assert_eq!(sum_square_difference(100), 25164150);
    }

    #[test]
    fn problem_7() {
        assert_eq!(nth_prime(6), 13);
        assert_eq!(nth_prime(10), 29);
        assert_eq!(nth_prime(100), 541);
        assert_eq!(nth_prime(1000), 7919);
        assert_eq!(nth_prime(10001), 104743);
    }


    #[test]
    fn problem_8() {
        assert_eq!(largest_product_in_series(4), 5832);
        assert_eq!(largest_product_in_series(13), 23514624000);
    }
}

