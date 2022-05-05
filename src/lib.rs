use emoji::lookup_by_glyph;
use unic_emoji_char::is_emoji;

pub use emoji::Emoji;

pub fn find_emoji(content: impl AsRef<str>) -> Vec<&'static Emoji> {
    let zwj = '\u{200d}';
    let mut emoji_list = vec![];
    let mut container = String::new();

    fn compose(container: &mut String, emoji_list: &mut Vec<&'static Emoji>) {
        if let Some(emoji) = lookup_by_glyph::lookup(container) {
            emoji_list.push(emoji);
        }
        container.clear();
    }

    let mut previous_zwj = false;
    for char in content.as_ref().chars() {
        if !previous_zwj {
            if !container.is_empty() && char == zwj {
                container.push(char);
                previous_zwj = true;
                continue;
            } else if !container.is_empty() && char != zwj {
                compose(&mut container, &mut emoji_list);
            }
        }
        previous_zwj = false;
        if is_emoji(char)
            || unic_emoji_char::is_emoji_component(char)
            || unic_emoji_char::is_emoji_modifier_base(char)
            || unic_emoji_char::is_emoji_modifier(char)
            || unic_emoji_char::is_emoji_presentation(char)
        {
            container.push(char);
        }
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
        let content = "@zuhairali83 😘❤️! 😻💓 👨‍👩‍👦  kk 👨‍👩‍👧‍👦";
        let emojis = find_emoji(content);
        assert_eq!(emojis.len(), 6);
        assert_eq!(emojis[0].name, "face blowing a kiss");
        assert_eq!(emojis[5].name, "family: man, woman, girl, boy");
    }
}
