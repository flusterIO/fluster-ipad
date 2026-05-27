use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use strum::IntoEnumIterator;
use typeshare::typeshare;

#[typeshare]
#[derive(Debug, Serialize, Deserialize, Clone, strum_macros::Display, strum_macros::EnumIter, PartialEq, Eq, Hash)]
pub enum MarkdownOutputVariantKey {
    #[serde(rename = "inline")]
    #[strum(to_string = "inline")]
    Inline,
    #[serde(rename = "forAI")]
    #[strum(to_string = "forAI")]
    ForAI,
    #[serde(rename = "general")]
    #[strum(to_string = "general")]
    General,
}

#[typeshare]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MarkdownComponentOptionMap<T: IntoEnumIterator + Debug + Clone>(pub  DashMap<MarkdownOutputVariantKey,
                                                                                        Option<T>>);

#[typeshare]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MarkdownComponentOptionData<T: IntoEnumIterator + Debug + Clone> {
    AsMap(MarkdownComponentOptionMap<T>),
    AsVariant(T),
}

#[typeshare]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MarkdownComponentOptions<T: IntoEnumIterator + Debug + Clone> {
    pub markdown: MarkdownComponentOptionData<T>,
}
