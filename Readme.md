# Emojito

Find Emoji in strings. Supports complex emoji such as 👨‍👩‍👧‍👦.
Uses the `unic-emoji-char` crate in the background, and does not rely on regexes.

## Usage

``` rs
let content = "Test 😘❤️! 😻💓 👨‍👩‍👦  kk 👨‍👩‍👧‍👦";
let emojis = emojito::find_emoji(content);
assert_eq!(emojis.len(), 6);
```
