use regex::Regex;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Word {
    pub word: String,
    pub replaced_words: HashSet<String>,
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
            replaced_words: HashSet::new(),
        }
    }

    pub fn replace(
        mut self,
        search_value: &Regex,
        replace_value: &str,
        replace_replaced_words: bool,
    ) -> Self {
        if !replace_replaced_words
            && self.search_value_contains_replaced_words(&search_value, &replace_value)
        {
            return self;
        }
        let mut replacing_word = self.word.clone();
        if search_value.is_match(&self.word) {
            replacing_word = search_value
                .replace_all(&self.word, replace_value)
                .parse()
                .expect("Failed to replace string.");
        }
        let collection = search_value.captures_iter(&self.word).collect::<Vec<_>>();
        let replaced_words: Vec<String> = if collection.len() > 1 {
            collection
                .into_iter()
                .map(|c| {
                    c.get(0)
                        .expect("Cannot get matched item at index 0 of the capture.")
                        .as_str()
                        .replace(
                            c.get(0)
                                .expect("Cannot get matched item at index 0 of the capture.")
                                .as_str(),
                            replace_value,
                        )
                })
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

    pub fn replace_with_func_single<F>(
        mut self,
        search_value: &Regex,
        func: F,
        replace_replaced_words: bool,
    ) -> Self
    where
        F: FnOnce() -> String,
    {
        let replace_value = func();

        if !replace_replaced_words
            && self.search_value_contains_replaced_words(&search_value, &replace_value)
        {
            return self;
        }

        let mut replacing_word = self.word.clone();
        if search_value.is_match(&self.word) {
            let match_item = search_value
                .captures(&self.word)
                .expect("Cannot get captures.")
                .get(0)
                .expect("Cannot get matched item at index 0 of the capture.")
                .as_str();

            replacing_word = self.word.replace(match_item, &replace_value);
        }

        let collection = search_value.captures_iter(&self.word).collect::<Vec<_>>();
        let replaced_words: Vec<String> = if collection.len() > 1 {
            collection
                .into_iter()
                .map(|c| {
                    c.get(0)
                        .expect("Cannot get matched item at index 0 of the capture.")
                        .as_str()
                        .replace(
                            c.get(0)
                                .expect("Cannot get matched item at index 0 of the capture.")
                                .as_str(),
                            &replace_value,
                        )
                })
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

    pub fn replace_with_func_multiple<F>(
        mut self,
        search_value: &Regex,
        func: F,
        replace_replaced_words: bool,
    ) -> Self
    where
        F: FnOnce(&str, &str) -> String,
    {
        if !search_value.is_match(&self.word) {
            return self;
        }

        let word = self.word.clone();
        let captures = search_value.captures(&word).expect("Cannot get captures.");
        let replace_value = func(
            captures
                .get(1)
                .expect("Cannot get matched item at index 1 of the capture.")
                .as_str(),
            captures
                .get(2)
                .expect("Cannot get matched item at index 2 of the capture.")
                .as_str(),
        );

        if !replace_replaced_words
            && self.search_value_contains_replaced_words(&search_value, &replace_value)
        {
            return self;
        }

        let replacing_word = self.word.replace(
            captures
                .get(0)
                .expect("Cannot get matched item at index 0 of the capture.")
                .as_str(),
            &replace_value,
        );
        let collection = search_value.captures_iter(&self.word).collect::<Vec<_>>();
        let replaced_words = if collection.len() > 1 {
            collection
                .into_iter()
                .map(|c| {
                    c.get(0)
                        .expect("Cannot get matched item at index 0 of the capture.")
                        .as_str()
                        .replace(
                            c.get(0)
                                .expect("Cannot get matched item at index 0 of the capture.")
                                .as_str(),
                            &replace_value,
                        )
                })
                .collect::<Vec<_>>()
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

    fn search_value_contains_replaced_words<'a>(
        &self,
        search_value: &'a Regex,
        replace_value: &'a str,
    ) -> bool {
        self.replaced_words.iter().any(|s| {
            if search_value.is_match(s.as_str()) {
                let match_result = search_value
                    .captures(s.as_str())
                    .expect("Cannot get captures.")
                    .get(0)
                    .expect("Cannot get matched item at index 0 of the capture.")
                    .as_str();
                return s.replace(match_result, replace_value) == *s;
            }
            false
        })
    }
}
