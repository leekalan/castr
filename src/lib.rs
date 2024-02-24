pub mod cast;

#[allow(unused_imports)]
use cast::{CastFrom, Castable};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn successful_test() {
        let string = "32";
        let num = string.cast_to::<u8>();
        assert_eq!(num, Some(32))
    }

    #[test]
    fn failing_test() {
        let string = "AB";
        let num = string.cast_to::<u8>();
        assert_eq!(num, None)
    }
}
