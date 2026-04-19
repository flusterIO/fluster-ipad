use askama::Template;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;
use winnow::{
    Parser,
    combinator::delimited,
    error::ErrMode,
    token::{literal, take_until},
};

use crate::{
    lang::{
        lib::ui::ui_types::inline_markdown_override::InlineMarkdownOverride,
        runtime::{
            state::{
                conundrum_error::ConundrumError,
                conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult},
                parse_state::{ConundrumCompileTarget, ConundrumModifier, ParseState},
            },
            traits::{
                conundrum_input::ConundrumInput, fluster_component_result::ConundrumComponentResult,
                html_js_component_result::HtmlJsComponentResult, markdown_component_result::MarkdownComponentResult,
                mdx_component_result::MdxComponentResult, plain_text_component_result::PlainTextComponentResult,
                state_modifier::ConundrumStateModifier,
            },
        },
    },
    output::{
        general::component_constants::{auto_inserted_component_name::AutoInsertedComponentName, parser_ids::ParserId},
        parsing_result::tag_result::TagResult,
    },
    parsers::parser_trait::ConundrumParser,
};

/// # Template (HTML)
///
/// ```askama
/// <span class="bg-primary hover:bg-primary/80 transition-colors duration-150 cursor-pointer text-primary-foreground rounded p-1 py-0.5">#{{body}}</span>
/// ```
#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize, Clone, Debug, Template)]
#[template(ext = "html", in_doc = true)]
pub struct ParsedTag {
    /// The tag's body
    pub body: String,
    /// The full match of the content that was originally in the note.
    pub full_match: String,
    pub markdown: Option<InlineMarkdownOverride>,
}

impl ConundrumStateModifier for ParsedTag {
    fn set_state(&self, res: &mut ParseState) {
        if !res.data.contains_tag(&self.body) {
            res.data.tags.push(TagResult { body: self.body.clone() });
        }
    }
}

impl PlainTextComponentResult for ParsedTag {
    fn to_plain_text(&self, _: &mut ParseState) -> ConundrumModalResult<String> {
        Ok(self.body.clone())
    }
}

impl ConundrumComponentResult for ParsedTag {
    fn to_conundrum_component(&self, res: &mut ParseState) -> ConundrumModalResult<String> {
        self.set_state(res);
        if res.compile_target == ConundrumCompileTarget::PlainText {
            self.to_plain_text(res)
        } else {
            self.to_mdx_component(res)
        }
    }
}

impl HtmlJsComponentResult for ParsedTag {
    fn to_html_js_component(&self, _: &mut ParseState) -> ConundrumModalResult<String> {
        self.render().map_err(|e| {
            eprintln!("Error: {:#?}", e);
            ErrMode::Cut(ConundrumErrorVariant::InternalParserError(ConundrumError::general_render_error()))
        })
    }
}

impl MarkdownComponentResult for ParsedTag {
    fn to_markdown(&self, res: &mut ParseState) -> ConundrumModalResult<String> {
        Ok(self.markdown.unwrap_or(InlineMarkdownOverride::Bold).wrap_content(&self.body))
    }
}

impl MdxComponentResult for ParsedTag {
    fn to_mdx_component(&self, res: &mut ParseState) -> ConundrumModalResult<String> {
        if res.data.ignore_all_parsers {
            return Ok(self.full_match.clone());
        }

        if res.data
              .front_matter
              .as_ref()
              .is_some_and(|fm| fm.ignored_parsers.iter().any(|x| x == &ParserId::NoteLink.to_string()))
        {
            return Ok(self.full_match.clone());
        }

        if res.data
              .front_matter
              .as_ref()
              .is_some_and(|fm| fm.ignored_parsers.iter().any(|x| x == &ParserId::Tags.to_string()))
        {
            return Ok(self.full_match.clone());
        }

        Ok(format!("<{}>{}</{}>",
                   AutoInsertedComponentName::AutoInsertedTag,
                   self.body,
                   AutoInsertedComponentName::AutoInsertedTag))
    }
}

impl ConundrumParser<ParsedTag> for ParsedTag {
    fn parse_input_string(input: &mut ConundrumInput) -> ConundrumModalResult<ParsedTag> {
        let (body, full_match) =
            delimited(literal("[[#"), take_until(1.., "]]"), literal("]]")).with_taken().parse_next(input)?;

        Ok(ParsedTag { body: body.to_string(),
                       full_match: full_match.to_string(),
                       markdown: None })
    }

    fn matches_first_char(char: char) -> bool {
        char == '['
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::wrap_test_content::wrap_test_conundrum_content;

    use super::*;

    #[test]
    fn parsed_link() {
        let test_content = "[[#myTag]]";

        let mut test_data = wrap_test_conundrum_content(test_content);

        let res = ParsedTag::parse_input_string(&mut test_data).expect("Parses outgoing link successfully.");

        assert!(test_data.input.is_empty(), "Citation returns the proper left over text.");

        assert!(res.body == "myTag", "Tag finds the proper body.");
    }
}
