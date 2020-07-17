use regex::{Regex, Captures};
use crate::utility::{
    OWO_MAPPING_LIST, SPECIFIC_WORD_MAPPING_LIST, UVU_MAPPING_LIST, UWU_MAPPING_LIST,
    interleave_arrays
};
use crate::structures::Word;
use std::collections::HashSet;

lazy_static! {
    static ref WORD_REGEX: Regex = Regex::new(r"[^\s]+").unwrap();
    static ref SPACE_REGEX: Regex = Regex::new(r"\s+").unwrap();
}

pub enum OwoifyLevel {
    Owo, Uwu, Uvu
}

pub trait Owoifiable {
    fn owoify(&self, level: &OwoifyLevel) -> String;
}

impl Owoifiable for String {
    fn owoify(&self, level: &OwoifyLevel) -> String {
        let word_matches = WORD_REGEX.captures_iter(self.as_str())
            .collect::<Vec<Captures>>();
        let space_matches = SPACE_REGEX.captures_iter(self.as_str())
            .collect::<Vec<Captures>>();

        let mut words = word_matches.into_iter()
            .map(|c| Word {
                word: String::from(c.get(0).unwrap().as_str()),
                replaced_words: HashSet::new()
            })
            .collect::<Vec<Word>>();

        let spaces = space_matches.into_iter()
            .map(|c| Word {
                word: String::from(c.get(0).unwrap().as_str()),
                replaced_words: HashSet::new()
            })
            .collect::<Vec<Word>>();

        words = words.into_iter()
            .map(|mut w| {
                for func in SPECIFIC_WORD_MAPPING_LIST.iter() {
                    func(&mut w);
                }

                match level {
                    OwoifyLevel::Owo => {
                        for func in OWO_MAPPING_LIST.iter() {
                            func(&mut w);
                        }
                    },
                    OwoifyLevel::Uwu => {
                        for func in UWU_MAPPING_LIST.iter() {
                            func(&mut w);
                        }
                        for func in OWO_MAPPING_LIST.iter() {
                            func(&mut w);
                        }
                    },
                    OwoifyLevel::Uvu => {
                        for func in UVU_MAPPING_LIST.iter() {
                            func(&mut w);
                        }
                        for func in UWU_MAPPING_LIST.iter() {
                            func(&mut w);
                        }
                        for func in OWO_MAPPING_LIST.iter() {
                            func(&mut w);
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