#![allow(unused_variables)]
#![allow(dead_code)]
#[macro_use]
extern crate lazy_static;

mod structures;
mod traits;
mod utility;
pub use traits::Owoifiable;
pub use traits::OwoifyLevel;

#[cfg(test)]
mod tests {
    use crate::{Owoifiable, OwoifyLevel};

    const SOURCE: &str = "Hello World! This is the string to owo! Kinda cute, isn't it?";

    #[test]
    fn test_owoify() {
        assert_ne!(SOURCE.owoify(&OwoifyLevel::Owo), SOURCE.to_string());
    }

    #[test]
    fn test_owo() {
        assert_ne!(SOURCE.owoify(&OwoifyLevel::Owo), "".to_string());
    }

    #[test]
    fn test_uwu() {
        assert_ne!(SOURCE.owoify(&OwoifyLevel::Uwu), "".to_string());
    }

    #[test]
    fn test_uvu() {
        assert_ne!(SOURCE.owoify(&OwoifyLevel::Uvu), "".to_string());
    }

    #[test]
    fn test_owo_not_equal_to_uwu() {
        assert_ne!(
            SOURCE.owoify(&OwoifyLevel::Owo),
            SOURCE.owoify(&OwoifyLevel::Uwu)
        );
    }

    #[test]
    fn test_owo_not_equal_to_uvu() {
        assert_ne!(
            SOURCE.owoify(&OwoifyLevel::Owo),
            SOURCE.owoify(&OwoifyLevel::Uvu)
        );
    }

    #[test]
    fn test_uwu_not_equal_to_uvu() {
        assert_ne!(
            SOURCE.owoify(&OwoifyLevel::Uwu),
            SOURCE.owoify(&OwoifyLevel::Uvu)
        );
    }
}
