use syntect::{highlighting::ThemeSet, html::highlighted_html_for_string, parsing::SyntaxSet};
use winnow::error::ErrMode;

use crate::{
    lang::runtime::state::{
        conundrum_error::ConundrumError,
        conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult},
    },
    parsers::markdown::code_block::general::general_codeblock::GeneralPresentationCodeBlock,
};

pub struct RenderCodeToHtmlReq {
    pub code: GeneralPresentationCodeBlock,
}

/// Renders codeblocks to html using syntec.
/// The `lang` property must be a valid syntec name.
pub fn render_general_codeblock_to_html(req: RenderCodeToHtmlReq) -> ConundrumModalResult<String> {
    let ss = match req.code.inline {
        true => SyntaxSet::load_defaults_newlines(),
        false => SyntaxSet::load_defaults_nonewlines(),
    };

    let syntax = ss.find_syntax_by_name(req.code.lang.to_string().as_str()).ok_or_else(|| {
            ErrMode::Cut(ConundrumErrorVariant::InternalParserError(ConundrumError::from_msg_and_details("Invalid syntax name", "Conundrum found an invalid syntax name, or a language that is not supported by the syntec highlighter. See the `Code??` documentation for more information.")))
    })?;

    let theme_sets = ThemeSet::load_defaults();
    let theme = &req.code.theme.unwrap_or_default().to_theme(&theme_sets);

    let x = highlighted_html_for_string(req.code.content.as_str(), &ss, syntax, theme).map_err(|e| {
        eprintln!("Error: {:#?}", e);
        ErrMode::Cut(ConundrumErrorVariant::InternalParserError(ConundrumError::from_msg_and_details("Invalid syntax name", "Conundrum failed to parse a provided codeblock.")))
    })?;

    Ok(x)
}

#[cfg(test)]
mod tests {
    use crate::parsers::markdown::code_block::{
        supported_languages::SupportedCodeBlockSyntax, supported_themes::SupportedCodeBlockTheme,
    };

    use super::*;

    #[test]
    fn renders_python_codeblock() {
        let test_content = r#"
@dataclass
class MyDataclass:
    y: Int = 2

    def __post_init__(self):
        print(self)
            "#;
        let res = render_general_codeblock_to_html(RenderCodeToHtmlReq { code: GeneralPresentationCodeBlock { content: test_content.to_string(), lang: SupportedCodeBlockSyntax::Python, theme: Some(SupportedCodeBlockTheme::SolarizedDark), inline: false } }).expect("Successfully highlights a python codeblock.");

        println!("Res: {:#?}", res);
    }
}
