pub mod euler;
pub use crate::euler::problemss_100::*;

fn main() {
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
}

