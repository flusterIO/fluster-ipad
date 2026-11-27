use std::collections::HashMap;

use conundrum_lang::{
    ecosystem::db::tables::DatabaseTable,
    lang::lib::ui::ui_types::emphasis::emphasis_model::Emphasis,
    lifted_models::primitives::case_insensitive_string::CaseInsensitiveString,
    output::html::web_specific_traits::css_value_representable::CSSVariablePairRepresentable,
    parsers::conundrum::color::{
        color_pair::ColorPair, color_scheme_group::ColorSchemeGroup, conundrum_color::ConundrumColor,
        css_color::CssColor,
    },
};
use conundrum_macros::DatabaseEntity;
use fake::Dummy;
use serde::{Deserialize, Serialize};
use specta::Type;
use strum::IntoEnumIterator;

use crate::vector::models::taggables::tag::Tag;

#[derive(Serialize, Deserialize, Clone, Debug, Dummy, Type, DatabaseEntity)]
#[db(table = DatabaseTable::ApplicationTheme)]
pub struct ApplicationTheme {
    #[db(primary)]
    pub theme_key: CaseInsensitiveString,
    pub tags: Vec<Tag>,
    pub foreground: ColorSchemeGroup<ConundrumColor>,
    pub background: ColorSchemeGroup<ConundrumColor>,
    pub popover: ColorSchemeGroup<ColorPair<ConundrumColor>>,
    pub muted: ColorSchemeGroup<ColorPair<ConundrumColor>>,
    pub destructive: ColorSchemeGroup<ColorPair<ConundrumColor>>,
    pub secondary: ColorSchemeGroup<ColorPair<ConundrumColor>>,
}

impl ApplicationTheme {
    pub fn get_emphasis_map(self) -> HashMap<Emphasis, ColorSchemeGroup<ColorPair<ConundrumColor>>> {
        let mut map = HashMap::new();
        for em in Emphasis::iter() {
            let color_group = em.to_css_variable_group();
            map.insert(em.clone(),
                       ColorSchemeGroup { light: ColorPair { background: ConundrumColor(color_group.background
                                                                                                   .clone()),
                                                             foreground: ConundrumColor(color_group.foreground
                                                                                                   .clone()) },
                                          dark: ColorPair { background: ConundrumColor(color_group.background),
                                                            foreground: ConundrumColor(color_group.foreground) } });
        }
        map.clone()
    }
}
