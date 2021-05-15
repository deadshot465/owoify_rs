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
    const POKEMON_NAME_LIST_PATH: &str = "assets/pokemons.txt";
    const WAR_AND_PEACE_PATH: &str = "assets/war_and_peace_chapter01-20.txt";

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

    #[test]
    fn test_pokemon_names() {
        let pokemons = std::fs::read_to_string(POKEMON_NAME_LIST_PATH)
            .expect("Failed to read pokemon names from the disk.");
        pokemons.split("\n").for_each(|name| {
            let name_with_owo = name.owoify(&OwoifyLevel::Owo);
            let name_with_uwu = name.owoify(&OwoifyLevel::Uwu);
            let name_with_uvu = name.owoify(&OwoifyLevel::Uvu);
            assert_ne!(name_with_owo, "".to_string());
            assert_ne!(name_with_uwu, "".to_string());
            assert_ne!(name_with_uvu, "".to_string());
        });
    }

    #[test]
    fn test_long_text() {
        let text = std::fs::read_to_string(WAR_AND_PEACE_PATH)
            .expect("Failed to read war and peace from the disk.");
        let text_with_owo = text.owoify(&OwoifyLevel::Owo);
        let text_with_uwu = text.owoify(&OwoifyLevel::Uwu);
        let text_with_uvu = text.owoify(&OwoifyLevel::Uvu);
        assert_ne!(text_with_owo, "".to_string());
        assert_ne!(text_with_uwu, "".to_string());
        assert_ne!(text_with_uvu, "".to_string());
    }
}
