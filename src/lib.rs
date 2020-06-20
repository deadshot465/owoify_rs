#[macro_use]
extern crate lazy_static;

use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::fmt;
use regex::{Regex, Captures};

lazy_static! {

}

pub trait Owoifiable {
    fn owoify(&self) -> String;
}

#[derive(Debug)]
struct Word {
    pub word: String,
    pub replaced_words: HashSet<String>
}

impl Display for Word {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.word)
    }
}

impl Word {
    pub fn new() -> Self {
        Word {
            word: String::new(),
            replaced_words: HashSet::new()
        }
    }

    pub fn replace<'a>(&self, search_value: &'a Regex, replace_value: &'a str, replace_replaced_words: bool) -> Self {
        let mut result = Word {
            word: self.word.clone(),
            replaced_words: self.replaced_words.clone()
        };
        if !replace_replaced_words &&
            self.search_value_contains_replaced_words(&search_value, &replace_value) {
            result
        }
        else {
            let mut replacing_word = self.word.clone();
            if search_value.is_match(self.word.as_str()) {
                replacing_word = search_value.replace_all(self.word.as_str(), replace_value).parse().unwrap();
            }
            let collection = search_value.captures(self.word.as_str()).unwrap();
            let replaced_words: Vec<String> = if collection.len() > 1 {
                collection.iter()
                    .map(|c| c.as_ref()
                        .unwrap()
                        .as_str()
                        .replace(c
                                     .as_ref()
                                     .unwrap()
                                     .as_str(), replace_value))
                    .collect()
            } else {
                vec![]
            };

            if replacing_word != self.word {
                for word in replaced_words.into_iter() {
                    result.replaced_words.insert(word);
                }
                result.word = replacing_word
            }
            result
        }
    }

    pub fn replace_with_func_single<F>(&self, search_value: &Regex, func: F, replace_replaced_words: bool) -> Self
    where F: FnOnce() -> String
    {
        let replace_value = func();
        let mut result = Word {
            word: self.word.clone(),
            replaced_words: self.replaced_words.clone()
        };

        if !replace_replaced_words &&
            self.search_value_contains_replaced_words(&search_value, &replace_value) {
            result
        }
        else {
            let mut replacing_word = self.word.clone();
            if search_value.is_match(self.word.as_str()) {
                let match_item = search_value
                    .captures(self.word.as_str())
                    .as_ref()
                    .unwrap()
                    .get(1)
                    .as_ref()
                    .unwrap()
                    .as_str();

                replacing_word = self.word.as_str().replace(match_item, replace_value.as_str());
            }
            let collection = search_value.captures(self.word.as_str()).unwrap();
            let replaced_words: Vec<String> = if collection.len() > 1 {
                collection.iter()
                    .map(|c| c.as_ref()
                        .unwrap()
                        .as_str()
                        .replace(c
                                     .as_ref()
                                     .unwrap()
                                     .as_str(), replace_value.as_str()))
                    .collect()
            } else {
                vec![]
            };

            if replacing_word != self.word {
                for word in replaced_words.into_iter() {
                    result.replaced_words.insert(word);
                }
                result.word = replacing_word
            }
            result
        }
    }

    pub fn replace_with_func_multiple<'a, F>(&'a mut self, search_value: &'a Regex, func: F, replace_replaced_words: bool) -> &'a Self
    where F: FnOnce(&'a str, &'a str) -> String
    {
        if !search_value.is_match(self.word.as_str()) {
            self
        }
        else {
            let captures = search_value.captures(self.word.as_str())
                .unwrap();
            let replace_value = func(captures
                .get(1)
                .unwrap()
                .as_str(),
            captures
                .get(2)
                .unwrap()
                .as_str());

            if !replace_replaced_words &&
                self.search_value_contains_replaced_words(&search_value, &replace_value) {
                self
            }
            else {
                let mut replacing_word = self.word
                    .replace(captures.get(0).unwrap().as_str(),replace_value.as_str());
                let collection = search_value.captures_iter(self.word.as_str())
                    .collect::<Vec<Captures>>();
                let replaced_words = if collection.len() > 0 {
                    collection.into_iter()
                        .map(|c| c
                            .get(0)
                            .unwrap()
                            .as_str()
                            .replace(c.get(0).unwrap().as_str(), replace_value.as_str()))
                        .collect::<Vec<String>>()
                } else {
                    vec![]
                };

                if replacing_word != self.word {
                    for word in replaced_words.into_iter() {
                        self.replaced_words.insert(word);
                    }
                    self.word = replacing_word;
                }
                self
            }
        }
    }

    fn search_value_contains_replaced_words<'a>(&self, search_value: &'a Regex, replace_value: &'a str) -> bool {
        self.replaced_words
            .iter()
            .any(|s| {
                if search_value.is_match(s.as_str()) {
                    let match_result = search_value.captures(s.as_str())
                        .unwrap()
                        .get(1)
                        .unwrap()
                        .as_str();
                    return s.replace(match_result, replace_value) == *s;
                }
                false
            })
    }
}

impl Owoifiable for String {
    fn owoify(&self) -> String {
        unimplemented!()
    }
}

impl Owoifiable for &str {
    fn owoify(&self) -> String {
        unimplemented!()
    }
}
