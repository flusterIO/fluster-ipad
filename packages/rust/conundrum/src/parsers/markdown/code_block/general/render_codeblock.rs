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
pub fn render_codeblock_to_html(req: RenderCodeToHtmlReq) -> ConundrumModalResult<String> {
    let ss = match req.code.inline {
        true => SyntaxSet::load_defaults_newlines(),
        false => SyntaxSet::load_defaults_nonewlines(),
    };

    let syntax = ss.find_syntax_by_name(req.code.lang.as_str()).ok_or_else(|| {
            ErrMode::Cut(ConundrumErrorVariant::InternalParserError(ConundrumError::from_msg_and_details("Invalid syntax name", "Conundrum found an invalid syntax name, or a language that is not supported by the syntec highlighter.")))
    })?;

    let theme_sets = ThemeSet::load_defaults();
    let theme = &theme_sets.themes["base16-ocean.dark"];

    let x = highlighted_html_for_string(req.code.content.as_str(), &ss, syntax, theme);

    Ok(String::from(""))
}

#[cfg(test)]
mod tests {
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
        let r = render_codeblock_to_html(RenderCodeToHtmlReq { code: GeneralPresentationCodeBlock { content: test_content.to_string(), lang: "python".to_string(), theme: Some("base16-ocean.dark".to_string()), inline: false } });
    }
}
