// https://github.com/rust-onig/rust-onig/blob/main/onig/examples/dollar.rs

use onig::{Captures, Replacer};
use std::borrow::Cow;

pub struct Dollarified<'a>(pub &'a str);

fn capture_str<'t>(caps: &'t Captures, cap_ref: &str) -> Option<&'t str> {
    cap_ref.parse::<usize>().ok().and_then(|p| caps.at(p))
}

impl<'a> Replacer for Dollarified<'a> {
    fn reg_replace(&mut self, caps: &Captures) -> Cow<str> {
        let mut replacement = String::new();
        let mut pattern = self.0;
        while !pattern.is_empty() {
            if let Some(position) = pattern.find('$') {
                replacement.push_str(&pattern[..position]);
                pattern = &pattern[position + 1..];

                let ref_end = pattern
                    .find(|c| !char::is_numeric(c))
                    .unwrap_or(pattern.len());

                if let Some(cap) = capture_str(caps, &pattern[..ref_end]) {
                    replacement.push_str(cap);
                    pattern = &pattern[ref_end..];
                }
                else {
                    replacement.push('$');
                }
            }
            else {
                replacement.push_str(pattern);
                break;
            }
        }
        replacement.into()
    }
}