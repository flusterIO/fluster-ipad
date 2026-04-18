use conundrum::lang::{
    lib::general::pagination::pagination_params::PaginationParams,
    runtime::{
        queries::{
            emojis::{search_emojis as search_emojis_func, EmojiSearchResults},
            get_title::{get_title_group, TitleGroup},
        },
        state::{
            conundrum_error_variant::ConundrumResult,
            parse_state::{ConundrumCompileTarget, ConundrumModifier},
        },
    },
};

#[uniffi::export()]
pub fn get_title_sync(content: String,
                      modifiers: Vec<ConundrumModifier>,
                      target: ConundrumCompileTarget)
                      -> ConundrumResult<TitleGroup> {
    get_title_group(content, modifiers, target)
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn get_title(content: String,
                       modifiers: Vec<ConundrumModifier>,
                       target: ConundrumCompileTarget)
                       -> ConundrumResult<TitleGroup> {
    get_title_group(content, modifiers, target)
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn search_emojis(query: String, pagination: Option<PaginationParams>) -> EmojiSearchResults {
    search_emojis_func(query, pagination)
}
