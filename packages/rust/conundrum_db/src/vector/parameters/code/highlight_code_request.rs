use conundrum::ecosystem::db::db_traits::async_traits::actionable_request::ActionableRequest;
use conundrum::ecosystem::error_handling::db_error::{DatabaseError, DatabaseResult};
use conundrum::output::general::output_variants::terminal_or_html::TerminalOrHtml;
use conundrum::parsers::markdown::code_block::general::general_codeblock::GeneralPresentationCodeBlock;
use conundrum::parsers::markdown::code_block::general::render_codeblock_to_html::{
    RenderCodeToHtmlReq, render_general_codeblock_to_html,
};
use conundrum::parsers::markdown::code_block::supported_languages::SupportedCodeBlockSyntax;
use conundrum::parsers::markdown::code_block::supported_themes::SupportedCodeBlockTheme;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type)]
pub struct HighlightCodeRequest {
    pub code: String,
    pub lang: SupportedCodeBlockSyntax,
    pub theme: SupportedCodeBlockTheme,
    pub inline: bool,
    // pub format: TerminalOrHtml,
}

impl Into<RenderCodeToHtmlReq> for HighlightCodeRequest {
    fn into(self) -> RenderCodeToHtmlReq {
        RenderCodeToHtmlReq { code: GeneralPresentationCodeBlock::new_with_new_assets(self.code.clone(),
                                                                                      self.lang.clone(),
                                                                                      Some(self.theme),
                                                                                      self.inline) }
    }
}

impl ActionableRequest<String> for HighlightCodeRequest {
    async fn execute_request(&self) -> DatabaseResult<String> {
        let req: RenderCodeToHtmlReq = self.clone().into();
        let res = render_general_codeblock_to_html(req).map_err(|e| {
                                                           log::error!("Error: {:?}", e);
                                                           DatabaseError::SerializationError
                                                       })?;
        Ok(res)
    }
}
