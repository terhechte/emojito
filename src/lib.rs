//! # Emojito
//!
//! Find Emoji in strings. Supports complex emoji such as 👨‍👩‍👧‍👦.
//! Uses the `unic-emoji-char` crate in the background, and does not rely on regexes.
//!
//! ## Usage
//!
//! ``` rs
//! let content = "Test 😘❤️! 😻💓 👨‍👩‍👦  kk 👨‍👩‍👧‍👦";
//! let emojis = emojito::find_emoji(content);
//! assert_eq!(emojis.len(), 6);
//! ```
use std::ops::Range;

use emojis::get as lookup_emoji;
pub use emojis::Emoji;
use icu_properties::sets;

/// Find all the emoji in a string. Returns the emoji in a `Vec`.
/// ``` rs
/// let content = "Test 😘❤️! 😻💓 👨‍👩‍👦  kk 👨‍👩‍👧‍👦";
/// let emojis = emojito::find_emoji(content);
/// assert_eq!(emojis.len(), 6);
/// ```
pub fn find_emoji(content: impl AsRef<str>) -> Vec<&'static Emoji> {
    find_emoji_ranges(content)
        .into_iter()
        .map(|(emoji, _)| emoji)
        .collect()
}

/// Find all the emoji in a string. Returns a struct containing
/// the range of the Emoji as well as the Emoji
/// ``` rs
/// let content = "Test 😘❤️! 😻💓 👨‍👩‍👦  kk 👨‍👩‍👧‍👦";
/// let emojis = emojito::find_emoji(content);
/// assert_eq!(emojis.len(), 6);
/// ```
fn find_emoji_ranges(
    content: impl AsRef<str>,
) -> impl ExactSizeIterator<Item = (&'static Emoji, Range<usize>)> {
    let zwj = '\u{200d}';
    let variation_selectors = ['\u{fe0f}', '\u{fe0e}'];
    let mut emoji_list = Vec::with_capacity(128);
    let mut container = String::with_capacity(8);

    let is_emoji_data = sets::load_emoji(&icu_testdata::unstable()).unwrap();
    let is_emoji_ref = is_emoji_data.as_borrowed();
    let is_emoji_component_data = sets::load_emoji_component(&icu_testdata::unstable()).unwrap();
    let is_emoji_component_ref = is_emoji_component_data.as_borrowed();
    let is_emoji_modifier_data = sets::load_emoji_modifier(&icu_testdata::unstable()).unwrap();
    let is_emoji_modifier_ref = is_emoji_modifier_data.as_borrowed();
    let is_emoji_modifier_base_data =
        sets::load_emoji_modifier_base(&icu_testdata::unstable()).unwrap();
    let is_emoji_modifier_base_ref = is_emoji_modifier_base_data.as_borrowed();
    let is_emoji_presentation_data =
        sets::load_emoji_presentation(&icu_testdata::unstable()).unwrap();
    let is_emoji_presentation_ref = is_emoji_presentation_data.as_borrowed();

    fn compose(
        position: usize,
        container: &mut String,
        emoji_list: &mut Vec<(&'static Emoji, Range<usize>)>,
    ) {
        if let Some(emoji) = lookup_emoji(container) {
            emoji_list.push((emoji, position..(position + container.len())));
        }
        container.clear();
    }

    let mut previous_zwj = false;
    let mut previous_emoji = false;
    let mut last_begin = 0usize;
    for (index, char) in content.as_ref().char_indices() {
        let is_emoji_presentation = is_emoji_presentation_ref.contains(char);
        let is_ascii = char.is_ascii();
        // Shortcut to ignore ascii characters which don't have a unicode presentation
        // for a good speed boost
        if !is_emoji_presentation && is_ascii {
            if !container.is_empty() {
                compose(last_begin, &mut container, &mut emoji_list);
            } else {
                last_begin = index;
            }
            continue;
        }
        // let is_emoji = unic_emoji_char::is_emoji(char);
        let is_emoji = is_emoji_ref.contains(char);
        let is_emoji_component = is_emoji_component_ref.contains(char);
        let is_emoji_modifier_base = is_emoji_modifier_base_ref.contains(char);
        let is_emoji_modifier = is_emoji_modifier_ref.contains(char);
        let is_emoji_variant = variation_selectors.contains(&char);
        if !previous_zwj {
            let is_empty = container.is_empty();
            if is_empty {
                last_begin = index;
            }
            // For Zero width joiners, we continue
            if !is_empty && (char == zwj || is_emoji_variant) {
                container.push(char);
                previous_zwj = true;
                continue;
            } else if !is_empty && char != zwj {
                // If this character is an emoji component and the previous character
                // was an emoji, don't compose just yet
                if !(previous_emoji && (is_emoji_component)) {
                    compose(last_begin, &mut container, &mut emoji_list);
                }
            }
        }
        previous_zwj = false;
        if is_emoji
            || is_emoji_component
            || is_emoji_modifier_base
            || is_emoji_modifier
            || is_emoji_presentation
        {
            if container.is_empty() {
                last_begin = index;
            }
            container.push(char);
        }
        previous_emoji = is_emoji;
    }
    if !container.is_empty() {
        compose(last_begin, &mut container, &mut emoji_list);
    }
    emoji_list.into_iter()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_all() {
        let content = "Test 😘❤️! 😻💓 👨‍👩‍👦  kk 👨‍👩‍👧‍👦";
        let emojis = find_emoji(content);
        assert_eq!(emojis.len(), 6);
        assert_eq!(emojis[0].name(), "face blowing a kiss");
        assert_eq!(emojis[5].name(), "family: man, woman, girl, boy");
    }

    #[test]
    fn fun_with_flags() {
        let content = "🇦🇩 🇪🇸";
        let emojis = find_emoji(content);
        assert_eq!(emojis.len(), 2);
    }

    #[test]
    fn test_ranges() {
        let content = "🇦🇩 🇪🇸Test ❤️ 😘 😻💓 👨‍👩‍👦  kk 👨‍👩‍👧‍👦";
        let emojis = find_emoji_ranges(content);
        for (emoji, range) in emojis {
            assert_eq!(&content[range.clone()], emoji.as_str());
        }
    }

    #[test]
    fn test_read_heart() {
        let content = "❤️";
        let emojis = find_emoji(content);
        assert_eq!(emojis.len(), 1);
        assert_eq!(emojis[0].as_str(), content);
    }

    #[test]
    fn test_new_emojis() {
        let content = "🫘 🫶 🫡 🫠";
        let emojis = find_emoji(content);
        assert_eq!(emojis.len(), 4);
    }
}
