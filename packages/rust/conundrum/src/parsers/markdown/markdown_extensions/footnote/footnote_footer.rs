use std::sync::Arc;

use serde::Serialize;
use tokio::io::repeat;
use winnow::{
    Parser,
    ascii::{space1, till_line_ending},
    combinator::opt,
    stream::Stream,
    token::take,
};

use crate::{
    lang::{
        lib::ui::ui_types::children::Children,
        runtime::{
            parse_conundrum_string::parse_elements,
            state::{conundrum_error_variant::ConundrumModalResult, parse_state::ConundrumModifier},
            traits::{
                conundrum_input::{ArcState, ConundrumInput},
                html_js_component_result::HtmlJsComponentResult,
                markdown_component_result::MarkdownComponentResult,
                plain_text_component_result::PlainTextComponentResult,
            },
        },
    },
    parsers::{
        conundrum::logic::number::conundrum_int::ConundrumInt,
        markdown::markdown_extensions::footnote::{
            footnote_anchor,
            footnote_result::{FootnoteResult, RenderedFootnoteResult},
        },
        parser_trait::ConundrumParser,
        parsers_shared::{
            indentation_handling::repeated_indented_lines::{join_indentend_line_types, repeated_indented_lines},
            shared_enums::line_terminator::{LineTerminator, until_line_terminator},
            space_or_tab::spaces_only,
        },
    },
};

#[typeshare::typeshare]
#[derive(Debug, Serialize, Clone)]
pub struct FootnoteFooter {
    pub idx: ConundrumInt,
    pub content: Children,
}

impl PlainTextComponentResult for FootnoteFooter {
    fn to_plain_text(&self, res: ArcState) -> ConundrumModalResult<String> {
        Ok(format!("{}) {}", self.idx, self.content.render(res)?))
    }
}

impl MarkdownComponentResult for FootnoteFooter {
    fn to_markdown(&self, res: ArcState) -> ConundrumModalResult<String> {
        let state = res.read_arc();
        if state.contains_modifier(&ConundrumModifier::PreferInlineMarkdownSyntax) {
            Ok(String::from(""))
        } else {
            Ok(format!("[^{}]: {}", self.idx, self.content.render(Arc::clone(&res))?))
        }
    }
}

impl ConundrumParser<FootnoteFooter> for FootnoteFooter {
    fn parse_input_string(input: &mut ConundrumInput) -> ConundrumModalResult<FootnoteFooter> {
        let start = input.input.checkpoint();
        '['.void().parse_next(input).inspect_err(|_| {
                                         input.input.reset(&start);
                                     })?;
        '^'.void().parse_next(input).inspect_err(|_| {
                                         input.input.reset(&start);
                                     })?;
        let n = ConundrumInt::parse_input_string.parse_next(input).inspect_err(|_| {
                                                                       input.input.reset(&start);
                                                                   })?;
        ']'.void().parse_next(input).inspect_err(|_| {
                                         input.input.reset(&start);
                                     })?;
        ':'.void().parse_next(input).inspect_err(|_| {
                                         input.input.reset(&start);
                                     })?;
        spaces_only(1).parse_next(input).inspect_err(|_| {
                                             input.input.reset(&start);
                                         })?;

        let (title_line_chars, _): (Vec<String>, LineTerminator) =
            until_line_terminator::<'_, String>(0.., |nested_input| {
                take(1usize).map(String::from).parse_next(nested_input)
            }).parse_next(input)
              .inspect_err(|_| {
                  input.input.reset(&start);
              })?;

        let _body = opt(repeated_indented_lines(1..)).parse_next(input)
                                                     .inspect_err(|_| {
                                                         input.input.reset(&start);
                                                     })?
                                                     .map(join_indentend_line_types);
        let mut s = String::from("");
        for c in title_line_chars {
            s += c.as_str();
        }
        s += "\n";
        if let Some(body) = _body {
            s += format!("{}\n", body).as_str();
        }
        let mut nested_input = ConundrumInput { input: &s,
                                                state: Arc::clone(&input.state) };
        let _children = parse_elements(&mut nested_input)?;
        let children = Children(_children);

        let mut state = input.state.write_arc();

        state.footnotes.apply_footnote_footer(&n, children.clone());
        drop(state);

        Ok(FootnoteFooter { idx: n,
                            content: children })
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
    fn parses_simple_footnote_footer() {
        let test_content = "[^1]: My footnote footer here.";
        let mut test_input = wrap_test_conundrum_content(test_content);
        let res = FootnoteFooter::parse_input_string.parse_next(&mut input)
                                                    .expect("Parses footnote footer without throwing an error.");
        assert!(res.content == "My footnote footer here.", "Matches content as expected");
        assert!(res.idx == 1, "Finds the proper footnote index.");
        // assert_eq!(result, 4);
    }
}
