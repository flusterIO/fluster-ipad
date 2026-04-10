use winnow::stream::AsChar;

use crate::lang::lib::ui::components::documentation::emoji::emoji_data::EmojiData;

pub fn format_emoji_name(name: &str) -> String {
    let mut chars: Vec<String> = Vec::new();
    for c in name.chars() {
        if c.is_space() {
            chars.push("_".to_string());
        } else if c == ':' {
            chars.push("".to_string());
        } else {
            chars.push(c.to_string().to_lowercase());
        }
    }

    let s = String::from_iter(chars);
    println!("S: {:#?}", s);
    s
}

pub fn search_emojis(query: String) -> Vec<EmojiData> {
    let mut items = Vec::new();
    for emoji in emoji::search::search_name(query.as_str()) {
        println!("Emoji: {:#?}", emoji.name);
        if let Some(svg) = twemoji_assets::svg::SvgTwemojiAsset::from_name(emoji.glyph) {
            items.push(EmojiData { name: emoji.name.to_string(),
                                   svg: svg.to_string() });
        } else {
            eprintln!("Found an emoji without a valid svg: {:#?}", emoji.glyph);
        }
    }

    items
}

pub fn get_supported_emoji_names() -> Vec<String> {
    // let names = emoji::
    // let names = twemoji_assets::svg::names::
    let mut items = Vec::new();
    // let item = emoji::food_and_drink::food_marine::CRAB.name;
    for (k, v) in emoji::lookup_by_name::iter_name_emoji() {
        if twemoji_assets::svg::SvgTwemojiAsset::from_name(format_emoji_name(k).as_str()).is_some() {
            items.push(v.name.to_string());
        } else {
            eprintln!("Found an emoji without a valid svg: {:#?}", format_emoji_name(v.name));
        }
    }

    items
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn searches_emojis() {
        let res = search_emojis("smile".to_string());
        assert!(!res.is_empty(),
                "Returns a non-empty array of emoji's. That's as far as I'm testing emoji's in an academic focused app... I feel ridiculous.")
    }

    #[test]
    fn gets_emoji_names() {
        let res = get_supported_emoji_names();
        println!("Res: {}", res.len());
        assert!(!res.is_empty(),
                "Returns a non-empty array of emoji's. That's as far as I'm testing emoji's in an academic focused app... I feel ridiculous.");
    }
}
