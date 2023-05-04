pub mod euler;
pub use crate::euler::problem_1::{multiples_of3and5};


//#[macro_use] extern crate itertools;


fn main() {
    let test_cases = [10, 49, 1000, 8456, 19564];
    for n in test_cases {
        println!("Result for {} is {}", n, multiples_of3and5(n));
    }
}


