use conundrum::lang::runtime::{
    queries::get_title::{get_title_group, TitleGroup},
    state::{conundrum_error_variant::ConundrumResult, parse_state::ConundrumModifier},
};

#[uniffi::export()]
pub fn get_title_sync(content: String, modifiers: Vec<ConundrumModifier>) -> ConundrumResult<TitleGroup> {
    get_title_group(content, modifiers)
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn get_title(content: String, modifiers: Vec<ConundrumModifier>) -> ConundrumResult<TitleGroup> {
    get_title_group(content, modifiers)
}
