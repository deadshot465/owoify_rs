use crate::structures::Word;
use crate::utility::{
    interleave_arrays, OWO_MAPPING_LIST, SPECIFIC_WORD_MAPPING_LIST, UVU_MAPPING_LIST,
    UWU_MAPPING_LIST,
};
use regex::Regex;
use std::borrow::Cow;
use std::collections::HashSet;
use std::string::FromUtf8Error;

lazy_static! {
    static ref WORD_REGEX: Regex =
        Regex::new(r"[^\s]+").expect("Failed to build regular expression.");
    static ref SPACE_REGEX: Regex =
        Regex::new(r"\s+").expect("Failed to build regular expression.");
}

/// The owoness level. Currently three levels are available (from lowest to highest): `Owo`, `Uwu`, `Uvu`.
#[derive(Copy, Clone, Debug)]
pub enum OwoifyLevel {
    Owo,
    Uwu,
    Uvu,
}

/// A trait that could be implemented to owoify any ASCII string to babyspeak gibberish using the given owoness level ([`OwoifyLevel`][owoifyLevel]).
///
/// [owoifyLevel]: OwoifyLevel
pub trait Owoifiable {
    type ResultType;

    /// Owoifies the source using the specified owoness level and returns a new `String`.
    fn owoify(&self, level: OwoifyLevel) -> Self::ResultType;

    /// Owoifies the source using `Uwu` owoness.
    fn uwuify(&self) -> Self::ResultType;

    /// Owoifies the source using `Uvu` owoness.
    fn uvuify(&self) -> Self::ResultType;
}

impl Owoifiable for String {
    type ResultType = String;

    /// Owoifies the given string using specified owoness level and returns a new `String`.
    /// # Examples
    /// ```rust
    /// use owoify_rs::{Owoifiable, OwoifyLevel};
    ///
    /// let source = "Hello, World! Rust is fun!".to_string();
    /// let result = source.owoify(OwoifyLevel::Owo);
    /// assert_ne!(source, result);
    /// ```
    fn owoify(&self, level: OwoifyLevel) -> Self::ResultType {
        self.as_str().owoify(level)
    }

    fn uwuify(&self) -> Self::ResultType {
        self.as_str().uwuify()
    }

    fn uvuify(&self) -> Self::ResultType {
        self.as_str().uvuify()
    }
}

impl Owoifiable for &str {
    type ResultType = String;

    /// Owoifies the given string literal using specified owoness level and returns a new `String`.
    /// # Examples
    /// ```rust
    /// use owoify_rs::{Owoifiable, OwoifyLevel};
    ///
    /// let result = "Hello, World! Rust is fun!".owoify(OwoifyLevel::Owo);
    /// assert_ne!("Hello, World! Rust is fun!", result);
    /// ```
    fn owoify(&self, level: OwoifyLevel) -> Self::ResultType {
        let word_matches = WORD_REGEX.captures_iter(self);
        let space_matches = SPACE_REGEX.captures_iter(self);

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

    fn uwuify(&self) -> Self::ResultType {
        self.owoify(OwoifyLevel::Uwu)
    }

    fn uvuify(&self) -> Self::ResultType {
        self.owoify(OwoifyLevel::Uvu)
    }
}

impl Owoifiable for Vec<u8> {
    type ResultType = Result<String, FromUtf8Error>;

    /// Owoifies the given string in bytes. The bytes are cloned and will return a new `String`.
    /// Returns a `Result<String, FromUtf8Error>`.
    fn owoify(&self, level: OwoifyLevel) -> Self::ResultType {
        let clone = self.clone();
        String::from_utf8(clone).map(|s| s.owoify(level))
    }

    fn uwuify(&self) -> Self::ResultType {
        let clone = self.clone();
        String::from_utf8(clone).map(|s| s.uwuify())
    }

    fn uvuify(&self) -> Self::ResultType {
        let clone = self.clone();
        String::from_utf8(clone).map(|s| s.uvuify())
    }
}

impl Owoifiable for &[u8] {
    type ResultType = String;

    /// Owoifies the given string in byte slice form with invalid sequences replaced with [`U+FFFD REPLACEMENT CHARACTER`][U+FFFD].
    /// Returns a new `String`.
    ///
    /// [U+FFFD]: core::char::REPLACEMENT_CHARACTER
    fn owoify(&self, level: OwoifyLevel) -> Self::ResultType {
        match String::from_utf8_lossy(self) {
            Cow::Borrowed(s) => s.owoify(level),
            Cow::Owned(s) => s.owoify(level),
        }
    }

    fn uwuify(&self) -> Self::ResultType {
        match String::from_utf8_lossy(self) {
            Cow::Borrowed(s) => s.uwuify(),
            Cow::Owned(s) => s.uwuify(),
        }
    }

    fn uvuify(&self) -> Self::ResultType {
        match String::from_utf8_lossy(self) {
            Cow::Borrowed(s) => s.uvuify(),
            Cow::Owned(s) => s.uvuify(),
        }
    }
}
