## 1.1.0
**This version changed some mappings in presets, so the same input probably won't return the same result as version 1.0.0.**

- Add more word mappings, so it's on par with mohan-cao's latest owoify.
- Clean up codes by removing variable declarations in mappings.
- Add uwuify and uvuify for convenience.

## 1.0.1
- Bump up to 1.0.1

## 1.0.0
- **(Breaking change)** Change `owoify` function to take a copy of `OwoifyLevel` instead since it's trivially copyable.
- **(Breaking change)** Change the visibilities of all internal functions and constants to `pub(crate)` instead of `pub`.
- Improve documentation.
- Pass doctests for newly added documentation.
- Add implementation of `Owoifiable` for `Vec<u8>` and `&[u8]`.

## 0.2.0
- Apply clippy, properly make use Rust's ownership, and implement proper builder pattern.

## 0.1.5
- Add more kaomojis.

## 0.1.4
- Bug fixes.

## 0.1.3
- Use lazy static for word and space regex. (Thanks to [maxjoehnk](https://github.com/maxjoehnk).)