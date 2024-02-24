pub mod cast;

#[allow(unused_imports)]
use cast::{CastFrom, Castable};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let string = "32";
        let num = string.cast_to::<String>().unwrap();
        println!("{}", num);
    }
}
