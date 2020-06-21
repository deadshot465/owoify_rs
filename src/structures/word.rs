use onig::{Captures, Regex};
use std::collections::HashSet;

#[derive(Debug)]
pub struct Word {
    pub word: String,
    pub replaced_words: HashSet<String>
}

impl ToString for Word {
    fn to_string(&self) -> String {
        self.word.clone()
    }
}

impl Word {
    pub fn new() -> Self {
        Word {
            word: String::new(),
            replaced_words: HashSet::new()
        }
    }

    pub fn replace(&mut self, search_value: &Regex, replace_value: &str, replace_replaced_words: bool) -> &Self {
        if !replace_replaced_words &&
            self.search_value_contains_replaced_words(&search_value, &replace_value) {
            return self;
        }
        let mut replacing_word = self.word.clone();
        if search_value.is_match(self.word.as_str()) {
            replacing_word = search_value.replace_all(self.word.as_str(), replace_value).parse().unwrap();
        }
        let collection = search_value.captures_iter(self.word.as_str())
            .collect::<Vec<Captures>>();
        let replaced_words: Vec<String> = if collection.len() > 1 {
            collection.into_iter()
                .map(|c| c
                    .at(0)
                    .unwrap()
                    .replace(c
                                 .at(0)
                                 .unwrap(), replace_value))
                .collect()
        } else {
            vec![]
        };

        if replacing_word != self.word {
            for word in replaced_words.into_iter() {
                self.replaced_words.insert(word);
            }
            self.word = replacing_word
        }
        self
    }

    pub fn replace_with_func_single<F>(&mut self, search_value: &Regex, func: F, replace_replaced_words: bool) -> &Self
        where F: FnOnce() -> String
    {
        let replace_value = func();

        if !replace_replaced_words &&
            self.search_value_contains_replaced_words(&search_value, &replace_value) {
            return self;
        }

        let mut replacing_word = self.word.clone();
        if search_value.is_match(self.word.as_str()) {
            let match_item = search_value
                .captures(self.word.as_str())
                .unwrap()
                .at(0)
                .unwrap();

            replacing_word = self.word
                .as_str()
                .replace(match_item, replace_value.as_str());
        }

        let collection = search_value.captures_iter(self.word.as_str())
            .collect::<Vec<Captures>>();
        let replaced_words: Vec<String> = if collection.len() > 1 {
            collection.into_iter()
                .map(|c| c
                    .at(0)
                    .unwrap()
                    .replace(c
                                 .at(0)
                                 .unwrap(), replace_value.as_str()))
                .collect()
        } else {
            vec![]
        };

        if replacing_word != self.word {
            for word in replaced_words.into_iter() {
                self.replaced_words.insert(word);
            }
            self.word = replacing_word
        }
        self
    }

    pub fn replace_with_func_multiple<F>(&mut self, search_value: &Regex, func: F, replace_replaced_words: bool) -> &Self
        where F: FnOnce(&str, &str) -> String
    {
        if !search_value.is_match(self.word.as_str()) {
            return self;
        }

        let word = self.word.clone();
        let captures = search_value.captures(word.as_str())
            .unwrap();
        let replace_value = func(captures
                                     .at(1)
                                     .unwrap(),
                                 captures
                                     .at(2)
                                     .unwrap());

        if !replace_replaced_words &&
            self.search_value_contains_replaced_words(&search_value, &replace_value) {
            return self;
        }

        let replacing_word = self.word
            .replace(captures.at(0).unwrap(),replace_value.as_str());
        let collection = search_value.captures_iter(self.word.as_str())
            .collect::<Vec<Captures>>();
        let replaced_words = if collection.len() > 1 {
            collection.into_iter()
                .map(|c| c
                    .at(0)
                    .unwrap()
                    .replace(c.at(0).unwrap(), replace_value.as_str()))
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

    fn search_value_contains_replaced_words<'a>(&self, search_value: &'a Regex, replace_value: &'a str) -> bool {
        self.replaced_words
            .iter()
            .any(|s| {
                if search_value.is_match(s.as_str()) {
                    let match_result = search_value.captures(s.as_str())
                        .unwrap()
                        .at(1)
                        .unwrap();
                    return s.replace(match_result, replace_value) == *s;
                }
                false
            })
    }
}