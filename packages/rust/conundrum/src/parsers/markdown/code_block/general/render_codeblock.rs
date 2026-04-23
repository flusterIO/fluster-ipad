use syntect::html::highlighted_html_for_string;
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
    let data = req.code.assets.lock();
    let ss = data.get_syntax_set().map_err(|e| {
                                       eprintln!("Error: {:#?}", e);
                                       ErrMode::Cut(
            ConundrumErrorVariant::InternalParserError(ConundrumError::from_message("Highlighter Error"))
        )
                                   })?;

    // let ss = match req.code.inline {
    //     true => SyntaxSet::load_defaults_newlines(),
    //     false => SyntaxSet::load_defaults_nonewlines(),
    // };

    let syntax = ss.find_syntax_by_name(req.code.lang.to_string().as_str()).ok_or_else(|| {
            ErrMode::Cut(ConundrumErrorVariant::InternalParserError(ConundrumError::from_msg_and_details("Invalid syntax name", "Conundrum found an invalid syntax name, or a language that is not supported by the syntec highlighter. See the `Code??` documentation for more information.")))
    })?;

    let theme = data.get_theme(req.code.theme.unwrap_or_default().to_string().as_str());

    let mut x = highlighted_html_for_string(req.code.content.as_str(), ss, syntax, theme).map_err(|e| {
        eprintln!("Error: {:#?}", e);
        ErrMode::Cut(ConundrumErrorVariant::InternalParserError(ConundrumError::from_msg_and_details("Invalid syntax name", "Conundrum failed to parse a provided codeblock.")))
    })?;

    if req.code.inline {
        x.replace_first("<pre", "<code class=\"not-prose text-sm px-1 py-0.5 rounded\" ");
        x.replace_last("</pre>", "</code>");
    }

    Ok(x)
}

#[cfg(test)]
mod tests {
    use crate::{
        lang::runtime::state::parse_state::ParseState,
        parsers::markdown::code_block::{
            supported_languages::SupportedCodeBlockSyntax, supported_themes::SupportedCodeBlockTheme,
        },
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
        let assets = ParseState::default().highlight_assets;
        let res = render_general_codeblock_to_html(RenderCodeToHtmlReq { code: GeneralPresentationCodeBlock { content: test_content.to_string(), lang: SupportedCodeBlockSyntax::Python, theme: Some(SupportedCodeBlockTheme::SolarizedDark), inline: false, assets } }).expect("Successfully highlights a python codeblock.");

        println!("Res: {:#?}", res);
    }
}
