use serde::Serialize;
use winnow::stream::AsChar;

use crate::lang::lib::{
    general::pagination::pagination_params::PaginationParams,
    ui::components::{
        attention::emoji::currently_supported_emoji_names::CURRENTLY_SUPPORTED_EMOJI_NAMES,
        documentation::emoji::emoji_data::EmojiData,
    },
};

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

    String::from_iter(chars)
}

#[typeshare::typeshare]
#[derive(Serialize, uniffi::Record)]
pub struct EmojiSearchResults {
    pub names: Vec<EmojiData>,
    /// The total number of matches
    pub total: u32,
}

// TODO: Move this to using the provided names as they don't all match and their
// are extra possibilities. You'll need to find a decent fuzzy matching library
// for Rust first.
pub fn search_emojis(query: String, pagination: Option<PaginationParams>) -> EmojiSearchResults {
    let mut items = Vec::new();
    for emoji in CURRENTLY_SUPPORTED_EMOJI_NAMES::search_name(query.as_str()) {
        if let Some(svg) = twemoji_assets::svg::SvgTwemojiAsset::from_name(&emoji) {
            items.push(EmojiData { name: emoji.to_string(),
                                   svg: svg.to_string() });
        } else {
            eprintln!("Found an emoji without a valid svg: {:#?}", emoji);
        }
    }

    if let Some(pag) = pagination {
        let start = (((pag.page - 1) * pag.per_page) as usize).max(0);
        let end = (start + pag.per_page as usize).min(items.len());
        let x = &items[start..end];
        EmojiSearchResults { names: x.to_vec(),
                             total: items.len() as u32 }
    } else {
        let total = items.len() as u32;
        EmojiSearchResults { names: items,
                             total }
    }
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
        let res = search_emojis("love".to_string(),
                                Some(PaginationParams { per_page: 50,
                                                        page: 1 }));
        assert!(!res.emojis.is_empty(),
                "Returns a non-empty array of emoji's. That's as far as I'm testing emoji's in an academic focused app... I feel ridiculous.")
    }

    #[test]
    fn gets_emoji_names() {
        let res = get_supported_emoji_names();
        assert!(!res.is_empty(),
                "Returns a non-empty array of emoji's. That's as far as I'm testing emoji's in an academic focused app... I feel ridiculous.");
    }
}
