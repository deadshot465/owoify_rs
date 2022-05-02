# Owoify-rs
Turning your worst nightmare into a crate package.

![Rust](https://github.com/deadshot465/owoify_rs/workflows/Rust/badge.svg)

- [Crate package](https://crates.io/crates/owoify_rs)

This is a Rust port of [mohan-cao's owoify-js](https://github.com/mohan-cao/owoify-js), which will help you turn any string into nonsensical babyspeak similar to LeafySweet's infamous Chrome extension.

Just like owoify-js, three levels are available for owoifying your texts:

1. **owo (default):** The most vanilla one.
2. **uwu:** The moderate one.
3. **uvu:** Litewawwy unweadabwal.

Please refer to the original [owoify-js repository](https://github.com/mohan-cao/owoify-js) for more information.

## Reason for development
While there is already an [owoify](https://crates.io/crates/owoify) crate on crates.io, it doesn't provide different levels of owoness, and the owoness is also lighter, which makes the resulting sentences far more readable. For any user who wants to use owoify, the less readable the sentence, the better in my humble opinion.

## Install instructions
Just like any crates, simply put this line inside your `Cargo.toml`:
```toml
[dependencies]
owoify_rs = "~1.0.0"
```
Alternatively, pull the crate directly from the repository:
```toml
[dependencies]
owoify_rs = { git = "https://github.com/deadshot465/owoify_rs", branch = "regex" }
```
Note that the reason for setting the branch and that the default branch is the `regex` branch is that the [onig](https://crates.io/crates/onig) crate seems to have some inconsistencies despite the regexes are the same.

## Usage
Owoify-rs is implemented as a trait, and is implemented for both `String` and `&str`.
```rust
use owoify_rs::{Owoifiable, OwoifyLevel};

fn main() {
    let owo_str = String::from("This is the string to owo! Kinda cute isn't it?");
    let uvu_str = String::from("This is the string to owo! Kinda cute isn't it?");
    println!("{}", owo_str.owoify(OwoifyLevel::Owo));
    println!("{}", uvu_str.owoify(OwoifyLevel::Uvu));

    // Output:
    // This is teh stwing two owo! Kinda cute isn't it?
    // fwis is teh stwing twowo owowowo (⌒ω⌒) Kinda cute isn't it?
}
```

## Limitations
Since the [regex](https://crates.io/crates/regex) crate doesn't support negative lookahead, and `onig` shows inconsistent behaviors despite the regexes being the same as when using `regex`, and [fancy-regex](https://crates.io/crates/fancy-regex) doesn't support `captures_iter()` method at moment, currently the negative lookahead parts of these three regexes are omitted.

See the source code of `src/utility/mappings.rs` to see commented out regexes.

## Disclaimer
This crate was written to help myself get used to Rust's syntaxes and writing Rust programs. Performance is **NOT** guaranteed.

## See also
- [owoify-js](https://github.com/mohan-cao/owoify-js) - The original owoify-js repository.
- [Owoify.Net](https://www.nuget.org/packages/Owoify.Net/1.0.1) - The C# port of Owoify written by me.
- [Owoify++](https://github.com/deadshot465/OwoifyCpp) - The C++ header-only port of Owoify written by me.
- [owoify_rs](https://crates.io/crates/owoify_rs) - The Rust port of Owoify written by me.
- [owoify-py](https://pypi.org/project/owoify-py/) - The Python port of Owoify written by me.
- [owoify_dart](https://pub.dev/packages/owoify_dart) - The Dart port of Owoify written by me.
- [owoify_rb](https://rubygems.org/gems/owoify_rb) - The Ruby port of Owoify written by me.
- [owoify-go](https://pkg.go.dev/github.com/deadshot465/owoify-go/v2) - The Go port of Owoify written by me.
- [owoifySwift](https://github.com/deadshot465/OwoifySwift) - The Swift port of Owoify written by me.
- [owoifyKt](https://search.maven.org/search?q=g:%22io.github.deadshot465%22%20AND%20a:%22owoifyKt%22) - The Kotlin port of Owoify written by me.
- [owoify_ex](https://hex.pm/packages/owoify_ex) - The Elixir port of Owoify written by me.
- [owoify_cr](https://crystalshards.org/shards/github/deadshot465/owoify_cr) - The Crystal port of Owoify written by me.
- [owoifynim](https://nimble.directory/pkg/owoifynim) - The Nim port of Owoify written by me.
- [owoify-clj](https://clojars.org/net.clojars.deadshot465/owoify-clj) - The Clojure port of Owoify written by me.
- [purescript-owoify](https://pursuit.purescript.org/packages/purescript-owoify) - The PureScript port of Owoify written by me.
- [owoify-hs](https://hackage.haskell.org/package/owoify-hs) - The Haskell port of Owoify written by me.
- [owoify_erl](https://hex.pm/packages/owoify_erl) - The Erlang port of Owoify written by me.