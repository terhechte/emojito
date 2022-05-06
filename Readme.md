
[![license](https://shields.io/badge/license-MIT-green)](https://github.com/terhechte/emojito/blob/main/LICENSE.md)
![Rust CI](https://github.com/terhechte/emojito/actions/workflows/ci.yml/badge.svg)
[crates.io]: https://crates.io/crates/emojito
[![Documentation](https://docs.rs/emojito/badge.svg)][docs]

# Emojito

Find Emoji in strings. Supports complex emoji such as 👨‍👩‍👧‍👦.
Uses the `unic-emoji-char` crate in the background, and does not rely on regexes.

## Usage

``` rs
let content = "Test 😘❤️! 😻💓 👨‍👩‍👦  kk 👨‍👩‍👧‍👦";
let emojis = emojito::find_emoji(content);
assert_eq!(emojis.len(), 6);
```
