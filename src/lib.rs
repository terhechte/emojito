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
use emoji::lookup_by_glyph;

pub use emoji::Emoji;

/// Find all the emoji in a string. Returns the emoji in a `Vec`.
/// ``` rs
/// let content = "Test 😘❤️! 😻💓 👨‍👩‍👦  kk 👨‍👩‍👧‍👦";
/// let emojis = emojito::find_emoji(content);
/// assert_eq!(emojis.len(), 6);
/// ```
pub fn find_emoji(content: impl AsRef<str>) -> Vec<&'static Emoji> {
    let zwj = '\u{200d}';
    let mut emoji_list = Vec::with_capacity(128);
    let mut container = String::with_capacity(8);

    fn compose(container: &mut String, emoji_list: &mut Vec<&'static Emoji>) {
        if let Some(emoji) = lookup_by_glyph::lookup(container) {
            emoji_list.push(emoji);
        }
        container.clear();
    }

    let mut previous_zwj = false;
    let mut previous_emoji = false;
    for char in content.as_ref().chars() {
        let is_emoji_presentation = unic_emoji_char::is_emoji_presentation(char);
        let is_ascii = char.is_ascii();
        // Shortcut to ignore ascii characters which don't have a unicode presentation
        // for a good speed boost
        if !is_emoji_presentation && is_ascii {
            if !container.is_empty() {
                compose(&mut container, &mut emoji_list);
            }
            continue;
        }
        let is_emoji = unic_emoji_char::is_emoji(char);
        let is_emoji_component = unic_emoji_char::is_emoji_component(char);
        let is_emoji_modifier_base = unic_emoji_char::is_emoji_modifier_base(char);
        let is_emoji_modifier = unic_emoji_char::is_emoji_modifier(char);
        if !previous_zwj {
            // For Zero width joiners, we continue
            if !container.is_empty() && char == zwj {
                container.push(char);
                previous_zwj = true;
                continue;
            } else if !container.is_empty() && char != zwj {
                // If this character is an emoji component and the previous character
                // was an emoji, don't compose just yet
                if !(previous_emoji && is_emoji_component) {
                    compose(&mut container, &mut emoji_list);
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
            container.push(char);
        }
        previous_emoji = is_emoji;
    }
    if !container.is_empty() {
        compose(&mut container, &mut emoji_list);
    }
    emoji_list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_all() {
        let content = "Test 😘❤️! 😻💓 👨‍👩‍👦  kk 👨‍👩‍👧‍👦";
        let emojis = find_emoji(content);
        assert_eq!(emojis.len(), 6);
        assert_eq!(emojis[0].name, "face blowing a kiss");
        assert_eq!(emojis[5].name, "family: man, woman, girl, boy");
    }

    #[test]
    fn fun_with_flags() {
        let content = "🇦🇩 🇪🇸";
        let emojis = find_emoji(content);
        assert_eq!(emojis.len(), 2);
    }
}
