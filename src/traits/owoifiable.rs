use crate::structures::Word;
use crate::utility::{
    interleave_arrays, OWO_MAPPING_LIST, SPECIFIC_WORD_MAPPING_LIST, UVU_MAPPING_LIST,
    UWU_MAPPING_LIST,
};
use regex::Regex;
use std::collections::HashSet;

lazy_static! {
    static ref WORD_REGEX: Regex =
        Regex::new(r"[^\s]+").expect("Failed to build regular expression.");
    static ref SPACE_REGEX: Regex =
        Regex::new(r"\s+").expect("Failed to build regular expression.");
}

#[derive(Copy, Clone, Debug)]
pub enum OwoifyLevel {
    Owo,
    Uwu,
    Uvu,
}

pub trait Owoifiable {
    fn owoify(&self, level: &OwoifyLevel) -> String;
}

impl Owoifiable for String {
    fn owoify(&self, level: &OwoifyLevel) -> String {
        let word_matches = WORD_REGEX.captures_iter(self.as_str());
        let space_matches = SPACE_REGEX.captures_iter(self.as_str());

        let mut words = word_matches
            .into_iter()
            .map(|c| Word {
                word: String::from(
                    c.get(0)
                        .expect("Cannot get matched item at index 0 of the capture.")
                        .as_str(),
                ),
                replaced_words: HashSet::new(),
            })
            .collect::<Vec<_>>();

        let spaces = space_matches
            .into_iter()
            .map(|c| Word {
                word: String::from(
                    c.get(0)
                        .expect("Cannot get matched item at index 0 of the capture.")
                        .as_str(),
                ),
                replaced_words: HashSet::new(),
            })
            .collect::<Vec<_>>();

        words = words
            .into_iter()
            .map(|mut w| {
                for func in SPECIFIC_WORD_MAPPING_LIST.iter() {
                    w = func(w);
                }

                match level {
                    OwoifyLevel::Owo => {
                        for func in OWO_MAPPING_LIST.iter() {
                            w = func(w);
                        }
                    }
                    OwoifyLevel::Uwu => {
                        for func in UWU_MAPPING_LIST.iter() {
                            w = func(w);
                        }
                        for func in OWO_MAPPING_LIST.iter() {
                            w = func(w);
                        }
                    }
                    OwoifyLevel::Uvu => {
                        for func in UVU_MAPPING_LIST.iter() {
                            w = func(w);
                        }
                        for func in UWU_MAPPING_LIST.iter() {
                            w = func(w);
                        }
                        for func in OWO_MAPPING_LIST.iter() {
                            w = func(w);
                        }
                    }
                };

                w
            })
            .collect();

        let result = interleave_arrays(words, spaces);
        let result_string: String = result.iter().map(ToString::to_string).collect();
        result_string
    }
}

impl Owoifiable for &str {
    fn owoify(&self, level: &OwoifyLevel) -> String {
        let owned = self.to_string();
        owned.owoify(level)
    }
}
